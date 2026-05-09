//Code for plaintext
use simple_ring::Polynomial;
use crate::BFV;

#[derive(Clone)]
pub struct BFVPlaintext {
    pub plain: Polynomial,
    pub len: usize,
}

impl BFVPlaintext {
    pub fn empty(bfv: &BFV) -> Self { //Function that, given the cipher (BFV) creates and empty Plaintext ( Plaintext = [0, 0, 0, 0, ... , 0] )
        Self { 
            plain: Polynomial::zeros(bfv.params.n),
            len: 0, //We define the len of datas to 0, because the plaintext is empty.
        }
    }

    pub fn push(mut self, to_push: &str) -> Self { //Method to push datas into a plaintext

        let encoded: Vec<u64> = to_push.as_bytes().iter().map(|&b| b as u64).collect(); //First, we encode the datas into a Vec of u64s

        let remaining = self.plain.coeffs.len() - self.len; //We calculate the remaining coeffs we can write

        let len_to_write = encoded.len().min(remaining); 

        for i in 0..len_to_write { //We write the empty coefficients with pushed ones
            self.plain.coeffs[self.len + i] = encoded[i];
        }

        self.len += len_to_write; //We finally update the len of the plaintext
        self
    }

    pub fn new(text: &str, bfv: &BFV) -> Self { //Function that creates a plaintext, given a text in ASCII or UTF-8
        let n = bfv.params.n;

        let mut coeffs: Vec<u64> = vec![0; n]; //We create an empty vector

        let bytes = text.as_bytes(); //We turn the text into bytes.

        let len_to_write = bytes.len().min(n); //We deduce the len to write

        for i in 0..len_to_write { //We write the coeffs
            coeffs[i] = bytes[i] as u64;
        }

        Self {
            plain: Polynomial::new(coeffs), //We finally create the Plaintext
            len: len_to_write, 
        }
    }

    pub fn new_from_coeffs(coeffs: Vec<u64>, bfv: &BFV) -> Self { //Function to create a plaintext, but directly from a Vec of u64s, not from a String
        Self { plain: Polynomial::new(coeffs), len: bfv.params.n }
    }

    pub fn decode(&self) -> String { //Method to decode a plaintext
        let vec = self.plain.coeffs.to_vec();
        let vec: Vec<u8> = vec.iter().map(|&b| b as u8).collect();
        str::from_utf8(&vec).unwrap().to_string()
    }   
    
}