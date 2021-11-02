use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number : u32){
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} push ups!",
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    }else{
        if random_number == 3{
            println!("Task a break today! Remember to stay hydrated!");
        }else{
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T>
    where T : Fn(u32) -> u32
{
        calculation : T,
        value : Option<u32>,
}

impl<T> Cacher<T>
    where T : Fn(u32) -> u32
{
    fn new(calculation : T) -> Cacher<T> {
        Cacher {
            calculation,
            value : None,
        }
    }

    fn value(&mut self, arg : u32) -> u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2); //어떤 값을 넣어도 제일처음에 들어간 값, 첫번째 값 이후에는 some(v)를 계속 return함.
    assert_eq!(v1,v2);
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
