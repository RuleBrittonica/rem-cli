/*
 * Use of this source code is governed by the MIT license that can be
 * found in the LICENSE file.
 */

package org.rust.ide.refactoring.extractFunction

import com.intellij.openapi.actionSystem.DataContext
import com.intellij.openapi.command.WriteCommandAction
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.diagnostic.logger
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VfsUtil
import com.intellij.psi.PsiElement
import com.intellij.psi.PsiFile
import com.intellij.psi.PsiParserFacade
import com.intellij.psi.search.LocalSearchScope
import com.intellij.psi.search.searches.ReferencesSearch
import com.intellij.psi.util.PsiTreeUtil;
import com.intellij.psi.util.elementType
import com.intellij.refactoring.RefactoringActionHandler
import com.intellij.refactoring.RefactoringBundle
import com.intellij.usageView.UsageInfo
import org.rust.ide.presentation.PsiRenderingOptions
import org.rust.ide.presentation.RsPsiRenderer
import org.rust.ide.presentation.renderTypeReference
import org.rust.ide.refactoring.RsRenameProcessor
import org.rust.ide.utils.GenericConstraints
import org.rust.ide.utils.import.RsImportHelper.importTypeReferencesFromTys
import org.rust.lang.core.psi.*
import org.rust.lang.core.psi.ext.*
import org.rust.lang.core.psi.impl.RsMethodCallImpl
import org.rust.lang.core.psi.impl.RsFunctionImpl
import org.rust.lang.core.psi.impl.*
import org.rust.lang.core.resolve.RsCachedImplItem
import org.rust.lang.core.types.*
import org.rust.lang.core.types.ty.*
import org.rust.openapiext.runWriteCommandAction
import org.apache.commons.io.IOUtils
import java.io.File
import java.util.concurrent.TimeUnit

class RsExtractFunctionHandler : RefactoringActionHandler {
    override fun invoke(project: Project, elements: Array<out PsiElement>, dataContext: DataContext?) {
        //this doesn't get called from the editor.
    }

    override fun invoke(project: Project, editor: Editor?, file: PsiFile?, dataContext: DataContext?) {
        if (file !is RsFile) return
        val start = editor?.selectionModel?.selectionStart
        val end = editor?.selectionModel?.selectionEnd
        if (start === null || end === null) return
        val config = RsExtractFunctionConfig.create(file, start, end) ?: return
        extractFunctionDialog(project, config) {
            dump: Boolean ->
            extractFunction(project, file, config, dump)
        }
    }

    private fun extractFunction(project: Project, file: PsiFile, config: RsExtractFunctionConfig, dump: Boolean) {
        project.runWriteCommandAction(
            RefactoringBundle.message("extract.method.title"),
            file
        ) {
            val psiFactory = RsPsiFactory(project)
            val extractedFunction = addExtractedFunction(project, config, psiFactory) ?: return@runWriteCommandAction
            replaceOldStatementsWithCallExpr(config, psiFactory)
            val parameters = config.valueParameters.filter { it.isSelected }
            renameFunctionParameters(extractedFunction, parameters.map { it.name })
            val types = (parameters.map { it.type } + config.returnValue?.type).filterNotNull()
            importTypeReferencesFromTys(extractedFunction, types)
            if (dump) {
                if (dumpMethodCallTypes(project, extractedFunction, file, dump)) {
                    LOG.info("dumped call types completed successfully")
                }
            } else {
                if (dumpMethodCallTypes(project, extractedFunction, file, dump)) {
                    LOG.info("dumped call types completed successfully")
                    if (nonLocalController(project, config, file)){
                        LOG.info("controller completed successfully")
                        if (borrow(project, config, file)) {
                            LOG.info("borrow completed successfully")
                            if (repairLifetime(config, file, project, psiFactory)){
                                LOG.info("repairer completed successfully")
                            }
                        }
                    }
                }
            }
        }
    }

    private fun execAndGetVal(cmd: Array<String>) : Int {
        val proc = Runtime.getRuntime().exec(cmd)
        val stderr_r = proc.errorStream.bufferedReader()
        val stdout_r = proc.inputStream.bufferedReader()
        val stderr_reader = Thread {
            try {
                var tmp : String? = stderr_r.readLine()
                while (tmp != null) {
                    LOG.info("STDERR>$tmp")
                    tmp = stderr_r.readLine()
                }
            } catch (e : Exception) {
            }
        }

        val stdout_reader = Thread {
            try {
                var tmp : String? = stdout_r.readLine()
                while (tmp != null) {
                    LOG.info("STDOUT>$tmp")
                    tmp = stdout_r.readLine()
                }
            } catch (e : Exception) {
            }
        }
        stderr_reader.start()
        stdout_reader.start()
        proc.waitFor(5, TimeUnit.MINUTES)
        stderr_reader.join()
        stdout_reader.join()
        return proc.exitValue()
    }

    private fun failRepair(backupFile: String, filePath: String) {
        val newFileTxt = File(filePath).readText(Charsets.UTF_8)
        LOG.info("at failure: $newFileTxt")
        val cmd = arrayOf("cp", filePath, "/tmp/debug-repair")
        val proc = Runtime.getRuntime().exec(cmd)
        proc.waitFor(5, TimeUnit.MINUTES)
        val cmd2 = arrayOf("cp", backupFile, filePath)
        val proc2 = Runtime.getRuntime().exec(cmd2)
        proc2.waitFor(5, TimeUnit.MINUTES)
    }

    private fun dumpMethodCallTypes(project: Project, extractFn: RsFunction, file: PsiFile, dump: Boolean) : Boolean {
        val fileParent = file.getContainingDirectory().getVirtualFile().getPath()
        val fileName = file.name
        val filePath = "$fileParent/$fileName"
        LOG.info("file path: $filePath")

        var bak = "/tmp/${fileName}-ij-extract.bk"
        if (dump) {
            bak = "${filePath}_ORIGINAL"
        }

        execAndGetVal(arrayOf("cp", filePath, bak))

        var dumpFileName = "/tmp/method_call_mutability.txt"
        if (dump) {
            dumpFileName = "${filePath}_MUTABLE_METHOD_CALLS"
        }

        val dumpFile = File(dumpFileName)
        dumpFile.writeText("")
        val begin = System.currentTimeMillis()
        val visitor = object : RsVisitor() {
            override fun visitElement(o: RsElement) {
                o.acceptChildren(this)
            }

            override fun visitBlock(o: RsBlock) {
                LOG.debug("elem: ${o.text}")
                LOG.debug("elem type: $o")
                for (stmt in o.getStmtList()) {
                    stmt.acceptChildren(this)
                }
            }

            override fun visitDotExpr(o: RsDotExpr) {
                LOG.debug("dot expr: ${o.text}")
                super.visitDotExpr(o)
                val methodCall = o.getMethodCall()
                val inferred = methodCall?.inference?.getResolvedMethodType(methodCall)

                val selfTy = inferred?.paramTypes?.get(0)
                if (selfTy != null && selfTy is TyReference) {
                    if (selfTy.mutability.isMut) {
                        dumpFile.appendText("${o.text}\n")
                    }
                }
            }
        }
        try {
            extractFn.acceptChildren(visitor)
            val end = System.currentTimeMillis()
            LOG.info("dump method call elapsed time in milliseconds success: ${end?.minus(begin)}")
        } catch (e: Exception) {
            val end = System.currentTimeMillis()
            LOG.info("dump method call elapsed time in milliseconds failure: ${end?.minus(begin)}")
            LOG.error("dump method call failed: $e")
            if (!dump) {
                extractionFailed(project) {
                    failRepair(bak, filePath)
                }
            }
            return false
        }
        return true
    }

    private fun nonLocalController(project: Project, config: RsExtractFunctionConfig, file: PsiFile) : Boolean {
        val name = config.name
        val parentFn = config.function
        val fileParent = file.getContainingDirectory().getVirtualFile().getPath()
        val fileName = file.name
        val filePath = "$fileParent/$fileName"
        LOG.info("file path: $filePath")

        val bak = "/tmp/${fileName}-ij-extract.bk"
        val cmd1 = arrayOf("cp", filePath, bak)
        val proc1 = Runtime.getRuntime().exec(cmd1)
        proc1.waitFor(5, TimeUnit.MINUTES)

        //write the extracted fn
        File(filePath).writeText(file.text)

        var success = false
        val cmd : Array<String> = arrayOf("controller", "run", filePath, filePath, parentFn!!.name!!, name)
        var end : Long? = null
        val begin = System.currentTimeMillis()
        try {
            val exitValue = execAndGetVal(cmd)
            end = System.currentTimeMillis()
            LOG.info("exit val $exitValue")
            if (exitValue == 0) {
                LOG.info("nclf elapsed time in milliseconds success: ${end?.minus(begin)}")
                success = true
            }
        } catch (e: Exception) {
            end = System.currentTimeMillis()
            LOG.info("exception $e")
            LOG.info("nclf elapsed time in milliseconds failure: ${end?.minus(begin)}")
        } finally {
            VfsUtil.markDirtyAndRefresh(false, true, true, file.getVirtualFile())
            if (!success) {
                LOG.info("bad exit val restoring file")
                LOG.info("nclf elapsed time in milliseconds failure: ${end?.minus(begin)}")
                extractionFailed(project) {
                    failRepair(bak, filePath)
                }
            }
            return success
        }
    }

    private fun borrow(project: Project, config: RsExtractFunctionConfig, file: PsiFile) : Boolean {
        val name = config.name
        val parentFn = config.function
        val fileParent = file.getContainingDirectory().getVirtualFile().getPath()
        val fileName = file.name
        val filePath = "$fileParent/$fileName"
        LOG.info("file path: $filePath")

        val bak = "/tmp/${fileName}-ij-extract.bk"

        val dumpFileName = "/tmp/method_call_mutability.txt"

        val cmd : Array<String> = arrayOf("borrower", "run", filePath, filePath, dumpFileName, parentFn!!.name!!, name!!, bak)
        var success = false
        val begin = System.currentTimeMillis()
        var end: Long? = null
        try {
            val exitValue = execAndGetVal(cmd)
            end = System.currentTimeMillis()
            LOG.info("exit val $exitValue")
            if (exitValue == 0) {
                success = true
                LOG.info("borrow elapsed time in milliseconds success: ${end?.minus(begin)}")
            }
        } catch (e: Exception) {
            end = System.currentTimeMillis()
            LOG.info("exception $e")
            LOG.info("borrow elapsed time in milliseconds failure: ${end?.minus(begin)}")
        } finally {
            VfsUtil.markDirtyAndRefresh(false, true, true, file.getVirtualFile())
            if (!success) {
                LOG.info("bad exit val restoring file")
                LOG.info("borrow elapsed time in milliseconds failure: ${end?.minus(begin)}")
                extractionFailed(project) {
                    failRepair(bak, filePath)
                }
            }
            return success
        }
    }

    private fun repairLifetimeUsingRustc(config: RsExtractFunctionConfig, file: PsiFile, psiFactory: RsPsiFactory) : Boolean {

        val name = config.name
        val parentFnName = config.function.identifier.text
        LOG.info("parent fn name: $parentFnName, name: $name")
        var parentFn : RsFunction? = null


        val fileParent = file.getContainingDirectory().getVirtualFile().getPath()
        val filePath = "$fileParent/${file.name}"

        val fileAfterBorrowTxt = File(filePath).readText(Charsets.UTF_8)
        val fileAfterBorrow = psiFactory.createPsiFile(fileAfterBorrowTxt)
        var newFn : RsFunction? = null
        val initVisitor = object : RsVisitor() {
            override fun visitFunction(fn: RsFunction) {
                super.visitFunction(fn)
                LOG.info("found fn: ${fn.identifier.text}")
                if (fn.identifier.text == name){
                    newFn = fn
                }

                if (fn.identifier.text == parentFnName){
                    parentFn = fn
                }
            }
        }
        fileAfterBorrow.acceptChildren(initVisitor)

        if (parentFn == null || newFn == null) {
            LOG.info("rustc repair failure--did not run cannot find caller and callee fn")
            return false
        }

        val fnTxt = "#[allow(dead_code)]\n${parentFn!!.text}\n${newFn!!.text}"
        val fileName = "/tmp/pre_repair_extract.rs"
        val newFileName = "/tmp/post_repair_extract.rs"
        val mainTxt = "\nfn main() {}"
        File(fileName).writeText("$fnTxt$mainTxt")
        var end : Long? = null
        var success = false
        val cmd : Array<String> = arrayOf("repairer", "run", name, fileName, newFileName, "loosest-bounds-first")
        val begin = System.currentTimeMillis()
        try {
            val exitValue = execAndGetVal(cmd)
            end = System.currentTimeMillis()
            LOG.info("exit val $exitValue")
            if (exitValue == 0) {
                val newFileTxt = File(newFileName).readText(Charsets.UTF_8)
                val newFile = psiFactory.createPsiFile(newFileTxt)
                val visitor = object : RsVisitor() {
                    override fun visitFunction(fn: RsFunction) {
                        super.visitFunction(fn)
                        LOG.info("found fn: ${fn.identifier.text}")
                        if (fn.identifier.text == name){
                            LOG.info("set new fn: ${fn.identifier.text}")
                            newFn!!.replace(fn)
                        }

                        if (fn.identifier.text == parentFnName){
                            LOG.info("set new parent fn: ${fn.identifier.text}")
                            parentFn!!.replace(fn)
                        }
                    }
                }
                newFile.acceptChildren(visitor)
                success = true
                LOG.info("repair elapsed time in milliseconds rustc success: ${end?.minus(begin)}")
                File(filePath).writeText("${fileAfterBorrow.text}")
            } else {
                LOG.info("repair elapsed time in milliseconds rustc failure: ${end?.minus(begin)}")
            }
        } catch (e: Exception) {
            end = System.currentTimeMillis()
            LOG.info("exception $e")
            LOG.info("repair rustc elapsed time in milliseconds failure: ${end?.minus(begin)}")
        } finally {
            return success
        }
    }

    private fun repairLifetimeUsingCargo(config: RsExtractFunctionConfig, file: PsiFile, project: Project) : Boolean {
        val name = config.name
        val fileParent = file.getContainingDirectory().getVirtualFile().getPath()
        val fileName = file.name
        val filePath = "$fileParent/$fileName"
        LOG.info("file path: $filePath")

        val bak = "/tmp/${fileName}-ij-extract.bk"

        val baseDir = project.getBaseDir().getPath()

        // base manifest path too long compile time
        // val herePath = project.getBaseDir().getPath()
        // val manifestPath = "$herePath/Cargo.toml"
        var here = file.getContainingDirectory()
        while (here.findFile("Cargo.toml") == null && here.getVirtualFile().getPath() != baseDir) {
            here = here.getParentDirectory()
        }
        var revert = true
        if (here.findFile("Cargo.toml") == null) {
            noLifetimeFixMode(project, "No Cargo manifest file was found (we looked recursively until $baseDir), so no Cargo repair was done.  Do you want to proceed with possibly incorrect lifetimes?") {
                revert = false
            }
            if (revert) {
                failRepair(bak, filePath)
            }
            return false
        }
        val herePath = here.getVirtualFile().getPath()
        val manifestPath = "$herePath/Cargo.toml"
        LOG.info("manifest: $manifestPath")
        val cmd : Array<String> = arrayOf("repairer", "cargo", filePath, manifestPath, name, "loosest-bounds-first")
        var success = false
        var end : Long? = null
        val begin = System.currentTimeMillis()
        try {
            val exitValue = execAndGetVal(cmd)
            end = System.currentTimeMillis()

            LOG.info("exit val $exitValue")
            if (exitValue == 0) {
                success = true
                LOG.info("repair elapsed time in milliseconds cargo success: ${end?.minus(begin)}")
            }
        } catch (e: Exception) {
            end = System.currentTimeMillis()
            LOG.info("exception $e")
            LOG.info("repair cargo elapsed time in milliseconds failure: ${end?.minus(begin)}")
        } finally {
            VfsUtil.markDirtyAndRefresh(false, true, true, file.getVirtualFile())
            if (!success) {
                LOG.info("bad exit val restoring file")
                LOG.info("repair elapsed time in milliseconds cargo failure: ${end?.minus(begin)}")
                noLifetimeFixMode(project, "Lifetime repairs using Cargo has failed.  Do you want to proceed with possibly incorrect lifetimes?") {
                    revert = false
                }
                if (revert) {
                    failRepair(bak, filePath)
                }
            }
            return success
        }
    }

    private fun repairLifetime(config: RsExtractFunctionConfig, file: PsiFile, project: Project, psiFactory: RsPsiFactory) : Boolean {
        val fileParent = file.getContainingDirectory().getVirtualFile().getPath()
        val fileName = file.name
        val filePath = "$fileParent/$fileName"
        LOG.info("file path: $filePath")

        var cargoSuccess = false

        // if (repairLifetimeUsingRustc(config, file, psiFactory)) {
        //     LOG.info("lifetime repair using rustc succeeded")
        //     LOG.info("running cargo anyways for testing timing")
        //     repairLifetimeUsingCargo(config, file, project)
        // } else {
        //     cargoMode(project) {
        //         cargoSuccess = repairLifetimeUsingCargo(config, file, project)
        //     }
        // }
        cargoSuccess = repairLifetimeUsingCargo(config, file, project)
        return cargoSuccess
    }

    private fun addExtractedFunction(
        project: Project,
        config: RsExtractFunctionConfig,
        psiFactory: RsPsiFactory
    ): RsFunction? {
        val owner = config.function.owner
        val function = psiFactory.createFunction(config.functionText)
        val psiParserFacade = PsiParserFacade.getInstance(project)
        return when {
            owner is RsAbstractableOwner.Impl && !owner.isInherent -> {
                val impl = findExistingInherentImpl(owner.impl) ?: createNewInherentImpl(owner.impl) ?: return null
                val members = impl.members ?: return null
                members.addBefore(psiParserFacade.createWhiteSpaceFromText("\n\n"), members.rbrace)
                members.addBefore(function, members.rbrace) as? RsFunction
            }
            else -> {
                val newline = psiParserFacade.createWhiteSpaceFromText("\n\n")
                val end = config.function.block?.rbrace ?: return null
                config.function.addAfter(function, config.function.addAfter(newline, end)) as? RsFunction
            }
        }
    }

    /**
     * Finds inherent impl corresponding to [traitImpl].
     * Impls at same tree level are checked (e.g. if [traitImpl] is top-level impl, then top-level impls are checked).
     */
    private fun findExistingInherentImpl(traitImpl: RsImplItem): RsImplItem? {
        check(traitImpl.traitRef != null)
        val cachedTraitImpl = RsCachedImplItem.forImpl(traitImpl)
        return (traitImpl.parent as? RsItemsOwner)
            ?.childrenOfType<RsImplItem>()
            ?.firstOrNull { impl ->
                val cachedImpl = RsCachedImplItem.forImpl(impl)
                val (_, generics, constGenerics) = cachedImpl.typeAndGenerics ?: return@firstOrNull false
                cachedImpl.isInherent && cachedImpl.isValid && !cachedImpl.isNegativeImpl
                    && generics.isEmpty() && constGenerics.isEmpty()  // TODO: Support generics
                    && cachedImpl.typeAndGenerics == cachedTraitImpl.typeAndGenerics
            }
    }

    private fun createNewInherentImpl(traitImpl: RsImplItem): RsImplItem? {
        val parent = traitImpl.parent
        val psiFactory = RsPsiFactory(parent.project)

        val typeReference = traitImpl.typeReference!!
        val constraints = GenericConstraints.create(traitImpl).filterByTypeReferences(listOf(typeReference))

        val renderer = RsPsiRenderer(PsiRenderingOptions())

        val typeParameters = constraints.buildTypeParameters()
        val typeText = renderer.renderTypeReference(typeReference)
        val whereClause = constraints.buildWhereClause()

        val text = "impl$typeParameters $typeText $whereClause{}"
        val newImpl = psiFactory.tryCreateImplItem(text) ?: return null

        val newImplCopy = parent.addAfter(newImpl, traitImpl) as RsImplItem
        parent.addBefore(psiFactory.createWhitespace("\n\n"), newImplCopy)
        return newImplCopy
    }

    /**
     * Original function signature and body are inserted at first.
     * Then it is necessary to change the names of original parameters to the real (renamed) parameters' names.
     */
    private fun renameFunctionParameters(function: RsFunction, newNames: List<String>) {
        val parameters = function.rawValueParameters
            .map { it.pat }
            .filterIsInstance(RsPatIdent::class.java)
            .map { it.patBinding }

        for ((parameter, newName) in parameters.zip(newNames)) {
            if (newName != parameter.name) {
                val parameterUsages = ReferencesSearch.search(parameter, LocalSearchScope(function)).findAll()
                val usageInfo = parameterUsages.map { UsageInfo(it) }.toTypedArray()
                RsRenameProcessor().renameElement(parameter, newName, usageInfo, null)
            }
        }
    }

    private fun replaceOldStatementsWithCallExpr(config: RsExtractFunctionConfig, psiFactory: RsPsiFactory) {
        val stmt = StringBuilder()
        if (config.returnValue?.exprText != null) {
            stmt.append("let mut ${config.returnValue.exprText} = ")
        }
        val firstParameter = config.parameters.firstOrNull()
        stmt.append(if (firstParameter != null && firstParameter.isSelf) {
            "self.${config.name}(${config.argumentsText})"
        } else {
            val type = when (config.function.owner) {
                is RsAbstractableOwner.Impl,
                is RsAbstractableOwner.Trait -> "Self"
                else -> null
            }
            "${if (type != null) "$type::" else ""}${config.name}(${config.argumentsText})"
        })
        if (config.isAsync) {
            stmt.append(".await")
        }
        config.elements.forEachIndexed { index, psiElement ->
            if (index == config.elements.lastIndex) {
                when (psiElement) {
                    is RsExpr -> psiElement.replace(psiFactory.createExpression(stmt.toString()))
                    is RsExprStmt -> {
                        val needsSemicolon = config.returnValue == null || config.returnValue.exprText != null
                        if (needsSemicolon) {
                            stmt.append(";")
                        }
                        psiElement.replace(psiFactory.createStatement(stmt.toString()))
                    }
                    is RsStmt -> {
                        stmt.append(";")
                        psiElement.replace(psiFactory.createStatement(stmt.toString()))
                    }
                }
            } else {
                psiElement.delete()
            }
        }
    }

    companion object {
        val LOG: Logger = logger<RsExtractFunctionHandler>()
    }
}