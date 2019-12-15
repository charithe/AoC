use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct Claim {
    pub id: u32,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

fn to_u32(input: CompleteStr) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(&input, 10)
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(number<CompleteStr, u32>, map_res!(take_while!(is_digit), to_u32));

named!(parse_claim<CompleteStr, Claim>,
    ws!(do_parse!(
       tag!("#") >>
       id: number >>
       tag!("@") >>
       left: number >>
       tag!(",") >>
       top: number >>
       tag!(":") >>
       width: number >>
       tag!("x") >>
       height: number >>
       (Claim{ id , left, top, width, height })
    ))
);

pub fn claim(input: &str) -> Option<Claim> {
    parse_claim(CompleteStr(input)).ok().map(|x| x.1)
}

#[test]
fn test_parse_claim() {
    assert_eq!(
        parse_claim(CompleteStr("#123 @ 3,2: 5x4")),
        Ok((
            CompleteStr(""),
            Claim {
                id: 123,
                left: 3,
                top: 2,
                width: 5,
                height: 4,
            }
        ))
    )
}
