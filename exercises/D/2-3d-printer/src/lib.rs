// run with `cargo test -- --nocapture` to see the prints from state transitions
use std::marker::PhantomData;

pub struct Printer3D<S> {
    _marker: PhantomData<S>,
}

/* States */
pub struct Idle;
pub struct Printing;
pub struct ProductReady;
pub struct Error;

/// The 3D printer encountered an error and needs resetting
pub enum ErrorState {}
/// The 3D printer is waiting for a job
pub enum IdleState {}
/// The 3D printer is currently printing
pub enum PrintingState {}
/// The 3D printed product is ready
pub enum ProductReadyState {}

impl Printer3D<Idle> {
    pub fn new() -> Self {
        println!("3D Printer initialized in Idle state");
        Self {
            _marker: PhantomData,
        }
    }

    pub fn start_printing(self) -> Printer3D<Printing> {
        println!("Starting print job... Transitioning to Printing state");
        Printer3D {
            _marker: PhantomData,
        }
    }
}

impl Printer3D<Printing> {
    pub fn check_progress(self) -> PrintingResult {
        println!("Checking printing progress...");

        // Simulate checking if out of filament
        if self.is_out_of_filament() {
            println!("Out of filament! Transitioning to Error state");
            PrintingResult::OutOfFilament(Printer3D {
                _marker: PhantomData,
            })
        } else if self.is_product_ready() {
            println!("Product is ready! Transitioning to ProductReady state");
            PrintingResult::ProductReady(Printer3D {
                _marker: PhantomData,
            })
        } else {
            println!("Still printing...");
            PrintingResult::StillPrinting(self)
        }
    }

    fn is_out_of_filament(&self) -> bool {
        // Simulate random filament check
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);

        hasher.finish() % 10 == 0 // 10% chance of being out of filament
    }

    fn is_product_ready(&self) -> bool {
        // Simulate random product ready check
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut hasher = DefaultHasher::new();
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            + 1)
        .hash(&mut hasher);

        hasher.finish() % 5 == 0 // 20% chance of product being ready
    }
}

pub enum PrintingResult {
    StillPrinting(Printer3D<Printing>),
    ProductReady(Printer3D<ProductReady>),
    OutOfFilament(Printer3D<Error>),
}

impl Printer3D<ProductReady> {
    pub fn retrieve_product(self) -> Printer3D<Idle> {
        println!("Product retrieved! Transitioning back to Idle state");
        Printer3D {
            _marker: PhantomData,
        }
    }
}

impl Printer3D<Error> {
    pub fn reset(self) -> Printer3D<Idle> {
        println!("Printer reset! Transitioning back to Idle state");
        Printer3D {
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_print() {
        let printer = Printer3D::new();

        let mut printer = printer.start_printing();

        loop {
            match printer.check_progress() {
                PrintingResult::StillPrinting(p) => {
                    printer = p;
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                PrintingResult::ProductReady(p) => {
                    p.retrieve_product().start_printing();
                    break;
                }
                PrintingResult::OutOfFilament(p) => {
                    p.reset().start_printing();
                    break;
                }
            }
        }
    }
}
