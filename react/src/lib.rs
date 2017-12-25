use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
// pub type CellID = usize;
pub type CallbackID = usize;

pub struct Reactor<'call, T> {
    cells: HashMap<CellID, Rc<RefCell<Cell<'call, T>>>>,
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellID {
    ValueCell(usize),
    ComputeCell(usize),
}

pub struct Cell<'call, T> {
    id: CellID,
    value: T,
    compute_func: Option<Box<Fn(&[T]) -> T + 'call>>,
    callbacks: HashMap<CallbackID, Box<FnMut(T) -> () + 'call>>,
    dependencies: Vec<Rc<RefCell<Cell<'call, T>>>>,
    subscribers: Vec<Rc<RefCell<Cell<'call, T>>>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'call, T: Debug + Copy + PartialEq> Reactor<'call, T> {
    pub fn new() -> Self {
        Reactor { cells: HashMap::new() }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        let cell_id = CellID::ValueCell(self.cells.len());
        self.cells.insert(
            cell_id,
            Rc::new(RefCell::new(Cell {
                id: cell_id,
                value: initial,
                compute_func: None,
                callbacks: HashMap::new(),
                dependencies: Vec::new(),
                subscribers: Vec::new(),
            })),
        );

        cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'call>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        if dependencies.iter().any(|c| !self.cells.contains_key(c)) {
            Err(())
        } else {
            let cell_id = CellID::ComputeCell(self.cells.len());
            let value = compute_func(&dependencies.iter().fold(
                &mut Vec::<T>::new(),
                |acc, cell_id| {
                    if let Some(v) = self.value(*cell_id) {
                        acc.push(v);
                    }
                    acc
                },
            ));
            let mut dependencies_cell = Vec::<Rc<RefCell<Cell<'call, T>>>>::new();
            dependencies.iter().fold(
                &mut dependencies_cell,
                |acc, cell_id| {
                    if let Some(cell) = self.cells.get(cell_id) {
                        acc.push(Rc::clone(cell));
                    }
                    acc
                },
            );

            let cell = Rc::new(RefCell::new(Cell {
                id: cell_id,
                value: value,
                compute_func: Some(Box::new(compute_func)),
                callbacks: HashMap::new(),
                dependencies: dependencies_cell,
                subscribers: Vec::new(),
            }));
            dependencies.iter().for_each(
                |cell_id| if let Some(s_cell) =
                    self.cells.get_mut(cell_id)
                {
                    s_cell.borrow_mut().subscribers.push(Rc::clone(&cell));
                },
            );
            self.cells.insert(cell_id, Rc::clone(&cell));

            Ok(cell_id)
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        if let Some(cell) = self.cells.get(&id) {
            match id {
                CellID::ValueCell(_) => Some(cell.borrow().value),
                CellID::ComputeCell(_) => {
                    let ref_cell = cell.borrow();
                    match ref_cell.compute_func {
                        Some(ref f) => {
                            Some(f(&ref_cell.dependencies.iter().fold(
                                &mut Vec::<T>::new(),
                                |acc, d_cell| {
                                    if let Some(value) = self.value(d_cell.borrow().id) {
                                        acc.push(value);
                                    }
                                    acc
                                },
                            )))
                        }
                        None => None,
                    }
                }
            }
        } else {
            None
        }
    }

    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarl y, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        match id {
            CellID::ComputeCell(_) => {
                return Err(());
            }
            CellID::ValueCell(_) => {
                if let Some(cell) = self.cells.get(&id) {
                    {
                        let mut mut_cell = cell.borrow_mut();
                        mut_cell.value = new_value;
                    }
                    // notice to subscribers and invoke callbacks
                    self.update_value_and_invoke_callbacks(&cell.borrow());
                } else {
                    return Err(());
                }
            }
        }

        Ok(())
    }

    fn update_value_and_invoke_callbacks(&self, cell: &Cell<T>) {
        cell.subscribers.iter().for_each(|s_cell| {
            {
                let mut mut_s_cell = s_cell.borrow_mut();
                let (is_changed, s_new_value) = self.update_value(&mut mut_s_cell);
                if is_changed {
                    Self::invoke_callbacks(&mut mut_s_cell, s_new_value);

                }
            }
            self.update_value_and_invoke_callbacks(&s_cell.borrow());
        });
    }

    fn invoke_callbacks(cell: &mut Cell<T>, new_value: T) {
        cell.callbacks.iter_mut().for_each(
            |(_, f)| { f(new_value); },
        );
    }

    fn update_value(&self, cell: &mut Cell<T>) -> (bool, T) {
        let mut values = Vec::<T>::new();
        cell.dependencies.iter().fold(&mut values, |acc, d_cell| {
            if let Some(value) = self.value(d_cell.borrow().id) {
                acc.push(value);
            }

            acc
        });
        let new_value = match cell.compute_func {
            Some(ref f) => f(&values),
            None => cell.value,
        };
        let old_value = cell.value;
        cell.value = new_value;

        (new_value != old_value, new_value)
    }


    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'call>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        if let Some(cell) = self.cells.get_mut(&id) {
            let mut mut_cell = cell.borrow_mut();
            let mut callback_id = mut_cell.callbacks.len();
            while mut_cell.callbacks.contains_key(&callback_id) {
                callback_id += 1;
            }
            mut_cell.callbacks.insert(callback_id, Box::new(callback));

            Ok(callback_id)
        } else {
            Err(())
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self, id: CellID, callback: CallbackID) -> Result<(), ()> {
        if let Some(cell) = self.cells.get_mut(&id) {
            let mut mut_cell = cell.borrow_mut();
            match mut_cell.callbacks.remove(&callback) {
                Some(_) => Ok(()),
                None => Err(()),
            }
        } else {
            Err(())
        }
    }
}
