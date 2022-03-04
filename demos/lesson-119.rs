extern crate core;

struct Employee<State> {
    name: String,
    state: State
}

/* a blanket implementation on the Employee struct. Here we're implementing for ALL generic Employee types.
Specifically, we need some kind of shared function between all of the States of Employee, to transition into the next state.

We're not borrowing self here. We need to do that, because that will invalidate the previous state. If we were just to borrow here,
then the existing state can still remain. However since we MOVE self here, we get this new Employee that we're creating and the old Employee whatever
state it may happen to be, is not going to be accessible anymore.

This impl means: For every single Employee, for every single State, we're gonna have these functions in this block, available:*/
impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state
        }
    }
}

struct Agreement;

/* Implementing functionality for an Employee when the current state is the Agreement state: */
impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Agreement
        }
    }
    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}
struct Signature;

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}
struct Training;
#[rustfmt::skip]
impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        if score >= 7 {
            Ok(self.transition(OnboardingComplete {score}))
        } else {
            Err(self.transition(FailedTraining {score}))
        }
    }
}
struct FailedTraining {
    score: u8,
}
struct OnboardingComplete {
    score: u8,
}

fn main() {
    let employee = Employee::new("Sara");
    let onboarded = employee.read_agreement().sign().train(6);
    match onboarded {
        Ok(emp) => println!("onboarding complete"),
        Err(emp) => println!("training failed, score: {}", emp.state.score)
    }
}

