use std::slice;

struct Runtime<I> {
  memory: Vec<u8>,
  data_pointer: usize,
  input: I,
  output_callback: Option<fn(u8) -> ()>,
}

impl<'a, I: Iterator<Item = &'a u8> + Default> Runtime<I> {
  fn new(memory_size: usize, output_callback: Option<fn(u8) -> ()>) -> Self {
    Self {
      memory: vec![0; memory_size],
      data_pointer: 0,
      input: I::default(),
      output_callback
    }
  }

  fn increment_data_pointer(&mut self) {
    self.data_pointer += 1;
  }

  fn decrement_data_pointer(&mut self) {
    self.data_pointer -= 1;
  }

  fn increment_processing_byte(&mut self) {
    self.memory[self.data_pointer] += 1;
  }

  fn decrement_processing_byte(&mut self) {
    self.memory[self.data_pointer] -= 1;
  }

  fn read_input_and_store(&mut self) { 
    self.memory[self.data_pointer] = *self
      .input
      .next()
      .expect("Input should yeld a new vaule, but received None. It might be empty as first or all data has been read.");
  }

  fn output_processing_byte(&self) {
    let data = self.memory[self.data_pointer];

    match &self.output_callback {
      Some(callback) => callback(data),
      None => println!("{}", data)
    };
  }
}

struct Interpreter<'a> {
  runtime: &'a mut Runtime<slice::Iter<'a, u8>>,
  instruction_pointer: usize,
}

impl<'a> Interpreter<'a> {
  fn new(runtime: &'a mut Runtime<slice::Iter<'a, u8>>) -> Self {
    Self {
      runtime,
      instruction_pointer: 0
    }
  }

  fn interpret(&mut self, code: &str, input: Option<&'a str>) {
    let code_bytes = code.as_bytes();

    // Check if the input is provided.
    // If yes, override the current input. If not, continue to use the current input or an empty iterator (I::default).
    if input.is_some() {
      // The unwrap call will never panic in this case
      self.runtime.input = input.unwrap().as_bytes().iter();
    }
    self.instruction_pointer = 0;
  
    while self.instruction_pointer < code_bytes.len() {
      match code_bytes[self.instruction_pointer] {
        b'>' => self.runtime.increment_data_pointer(),
        b'<' => self.runtime.decrement_data_pointer(),
        b'+' => self.runtime.increment_processing_byte(),
        b'-' => self.runtime.decrement_processing_byte(),
        b'.' => self.runtime.output_processing_byte(),
        b',' => self.runtime.read_input_and_store(),
        _ => todo!("Handle characters that are not instruction")
      }

      self.instruction_pointer += 1;
    }
  }
}

fn main() {
  let mut r = Runtime::new(1, Some(|x|{println!("output: {}",x)}));
  let mut i = Interpreter::new(&mut r);

  i.interpret(",.", None);
}
