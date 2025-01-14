use super::*;

fn traits(ordinal: u64) -> Result<BTreeSet<String>> {
  Ok(
    Test::new()?
      .args(&["traits", &ordinal.to_string()])
      .ignore_stdout()
      .output()?
      .stdout
      .lines()
      .map(str::to_owned)
      .collect(),
  )
}

#[test]
fn invalid_ordinal() -> Result {
  Test::new()?
    .args(&["traits", "2099999997690000"])
    .expected_stderr("error: Invalid ordinal\n")
    .expected_status(1)
    .run()
}

#[test]
fn even() -> Result {
  assert!(traits(0)?.contains("even"));
  assert!(!traits(1)?.contains("even"));
  assert!(traits(2)?.contains("even"));
  Ok(())
}

#[test]
fn odd() -> Result {
  assert!(!traits(0)?.contains("odd"));
  assert!(traits(1)?.contains("odd"));
  assert!(!traits(2)?.contains("odd"));
  Ok(())
}

#[test]
fn pi() -> Result {
  assert!(!traits(0)?.contains("pi"));
  assert!(traits(3)?.contains("pi"));
  assert!(traits(31)?.contains("pi"));
  assert!(traits(314)?.contains("pi"));
  assert!(!traits(3145)?.contains("pi"));
  Ok(())
}

#[test]
fn nice() -> Result {
  assert!(!traits(0)?.contains("nice"));
  assert!(traits(69)?.contains("nice"));
  assert!(traits(6969)?.contains("nice"));
  assert!(traits(696969)?.contains("nice"));
  assert!(!traits(696968)?.contains("nice"));
  assert!(!traits(6969698)?.contains("nice"));
  Ok(())
}

#[test]
fn divine() -> Result {
  assert!(!traits(0)?.contains("angelic"));
  assert!(traits(7)?.contains("angelic"));
  assert!(traits(77)?.contains("angelic"));
  assert!(traits(777)?.contains("angelic"));
  assert!(!traits(778)?.contains("angelic"));
  Ok(())
}

#[test]
fn name() -> Result {
  assert!(traits(2099999997689999)?.contains("name: a"));
  assert!(traits(2099999997689999 - 1)?.contains("name: b"));
  assert!(traits(2099999997689999 - 25)?.contains("name: z"));
  assert!(traits(2099999997689999 - 26)?.contains("name: aa"));
  assert!(traits(0)?.contains("name: nvtdijuwxlp"));
  assert!(traits(1)?.contains("name: nvtdijuwxlo"));
  assert!(traits(26)?.contains("name: nvtdijuwxkp"));
  assert!(traits(27)?.contains("name: nvtdijuwxko"));
  Ok(())
}

#[test]
fn height() -> Result {
  assert!(traits(0)?.contains("height: 0"));
  assert!(traits(1)?.contains("height: 0"));
  assert!(traits(50 * 100_000_000)?.contains("height: 1"));
  assert!(traits(2099999997689999)?.contains("height: 6929999"));
  assert!(traits(2099999997689998)?.contains("height: 6929998"));
  Ok(())
}

#[test]
fn epoch() -> Result {
  assert!(traits(0)?.contains("epoch: 0"));
  assert!(traits(1)?.contains("epoch: 0"));
  assert!(traits(50 * 100_000_000 * 210000)?.contains("epoch: 1"));
  assert!(traits(2099999997689999)?.contains("epoch: 32"));
  Ok(())
}

#[test]
fn lucky() -> Result {
  assert!(traits(0)?.contains("luck: 0/1"));
  assert!(traits(8)?.contains("luck: 1/1"));
  assert!(traits(88)?.contains("luck: 2/2"));
  assert!(traits(89)?.contains("luck: 1/2"));
  assert!(traits(84)?.contains("luck: 0/2"));
  assert!(traits(4)?.contains("luck: -1/1"));
  Ok(())
}

#[test]
fn shiny() -> Result {
  assert!(traits(0)?.contains("shiny"));
  assert!(!traits(1)?.contains("shiny"));
  assert!(traits(2099999997689999)?.contains("shiny"));
  assert!(traits(2099999997689998)?.contains("shiny"));
  assert!(traits(50 * 100_000_000)?.contains("shiny"));
  assert!(!traits(50 * 100_000_000 + 1)?.contains("shiny"));
  Ok(())
}

#[test]
fn population() -> Result {
  assert!(traits(0)?.contains("population: 0"));
  assert!(traits(1)?.contains("population: 1"));
  assert!(traits(2)?.contains("population: 1"));
  assert!(traits(3)?.contains("population: 2"));
  assert!(traits(4)?.contains("population: 1"));
  Ok(())
}

#[test]
fn square() -> Result {
  assert!(traits(0)?.contains("square"));
  assert!(traits(1)?.contains("square"));
  assert!(!traits(2)?.contains("square"));
  assert!(traits(4)?.contains("square"));
  assert!(!traits(5)?.contains("square"));
  assert!(traits(9)?.contains("square"));
  Ok(())
}

#[test]
fn cube() -> Result {
  assert!(traits(0)?.contains("cube"));
  assert!(traits(1)?.contains("cube"));
  assert!(!traits(2)?.contains("cube"));
  assert!(traits(8)?.contains("cube"));
  assert!(!traits(9)?.contains("cube"));
  assert!(traits(27)?.contains("cube"));
  Ok(())
}

#[test]
fn character() -> Result {
  assert!(traits(0x000000)?.contains("character: '\\0'"));
  assert!(traits(0x000041)?.contains("character: 'A'"));
  assert!(traits(0x01F602)?.contains("character: '😂'"));
  assert!(traits(0x110000)?.contains("character: '\\0'"));
  assert!(traits(0x110041)?.contains("character: 'A'"));
  Ok(())
}

#[test]
fn cursed() -> Result {
  assert!(!traits(0)?.contains("cursed"));
  assert!(!traits(623624999999999)?.contains("cursed"));
  assert!(traits(623624999999999 - 1)?.contains("cursed"));
  assert!(!traits(623624999999999 + 1)?.contains("cursed"));
  Ok(())
}

#[test]
fn illusive() -> Result {
  assert!(!traits(1476374997689999 - 1)?.contains("illusive"));
  assert!(traits(623624999999999)?.contains("illusive"));
  assert!(!traits(623624999999999 + 1)?.contains("illusive"));
  Ok(())
}
