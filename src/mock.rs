type Tid = u32;
type TName<'a> = &'a str;
type TAge = u8;
type TEmpCode = u8;
type TComp = u32;
type Trow<'a> = (Tid, TName<'a>, TAge, TEmpCode, TComp);

pub fn db<'a>() -> &'static [Trow<'a>] {
    &[
        (1, "Jack", 20, 3, 36_000),
        (2, "Molly", 23, 8, 76_000),
        (3, "Steve", 27, 18, 56_000),
        (4, "Pete", 33, 1, 126_000)
    ]
}
