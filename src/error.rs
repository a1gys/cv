// pub struct ImageError {
//     message: String,
//     error_type: ErrorType,
// }

pub enum ErrorType {
    PixelInvalidValue,
}

// impl Error {
//     fn error_message(&mut self, error_type: ErrorType) -> ErrorType {
//         match error_type {
//             ErrorType::PixelInvalidValue => {
//                 self.message = String::from("Pixel value is out of range");
//                 self.error_type = ErrorType::PixelInvalidValue;
//                 return ErrorType::PixelInvalidValue
//             },
//             _ => todo!("Not implemented yet!"),
//         }
//     }
// }