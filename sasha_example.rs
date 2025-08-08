struct Dummy;

impl Dummy {
    fn populate_loose<'a, 'b, 'c, 'd>(&'a self, f: &'b mut impl FnMut(&'c str, &'d str)) {
        f("foo", "loose_val");
    }

    fn populate_packed<'a, 'b, 'c, 'd>(&'a self, f: &'b mut impl FnMut(&'c str, &'d str)) {
        f("foo", "packed_val");
    }

    fn lookup(&self, name: &str) -> Option<&'static str> {
        let mut result = None;

        {
            let insert = &mut |n: &str, v: &'static str| {
                if n == name {
                    result = Some(v);
                }
            };
            self.populate_loose(insert);
        }

        if result.is_none() {
            let insert = &mut |n: &str, v: &'static str| {
                if n == name {
                    result = Some(v);
                }
            };
            self.populate_packed(insert);
        }

        result
    }

    fn lookup_2(&self, name: &str) -> Option<&'static str> {
        let mut result = None;

        let insert = &mut |n: &str, v: &'static str| {
            if n == name {
                result = Some(v);
            }
        };

        self.populate_loose(insert);

        if result.is_none() {
            self.populate_packed(insert);
        }

        result
    }

}

fn main() {
    let d = Dummy;
    let res = d.lookup("foo");
    println!("{res:?}");
}
