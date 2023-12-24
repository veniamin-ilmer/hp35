use boards::hp35::Board;

pub(super) struct Display {
  display: web_sys::Element,
  current_str: String,
}

impl Display {
  pub(super) fn new() -> Self {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let display = document.get_element_by_id("display").unwrap();
    
    Self {
      display,
      current_str: String::from("               "),
    }
  }
  
  pub(super) fn run_refresh_cycle(&mut self, board: &Board) {
    let mut buffer = Vec::with_capacity(15);
    let new_str = if board.anr.display_on {
      let mut a = board.anr.a.clone();
      let mut b = board.anr.b.clone();
      let sign = a.get_nibble(true);
      buffer.push(if sign.value() == 9 { '-' } else { ' ' });
      a.rotate_with_nibble(sign, true);
      for location in 0..14 {
        let mask = b.get_nibble(true);
        if mask.value() == 2 {
          buffer.push('.');
          b.rotate_with_nibble(mask, true);
          continue;
        }
        let digit = a.get_nibble(true);
        buffer.push(match mask.value() {
          9 => ' ',
          _ => if location == 11 {
            if digit.value() == 9 { '-' } else { ' ' }
          } else {
            (digit.value() + 48) as char
          },
        });
        a.rotate_with_nibble(digit, true);
        b.rotate_with_nibble(mask, true);
      }
      buffer.iter().collect::<String>()
    } else {
      String::from("               ")
    };
    if self.current_str != new_str {
      self.display.set_text_content(Some(&new_str));
      self.current_str = new_str.clone();
    }
  }
}