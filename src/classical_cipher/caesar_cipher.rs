
pub fn caesar_cipher(input:String, key:u8, is_encrypt:bool) -> String{

   let mut output: String = String::new();

   if is_encrypt{
      for ch in input.chars(){
         println!("{}", (ch.to_ascii_lowercase() as u8));
         let ascii_ch: u8 = (ch.to_ascii_lowercase() as u8) - 97;
         let result: u8 = ((ascii_ch + key) % 26) + 97;
         output.push(result as char);
      }
   }else{
      for ch in input.chars(){
         let ascii_ch: u8 = (ch.to_ascii_lowercase() as u8) - 97;
         let result: u8 = ((ascii_ch - key) % 26) + 97;
         output.push(result as char);
      }
   }
   return output;

}