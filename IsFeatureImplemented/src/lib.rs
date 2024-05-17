#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use num_of_Feature::*;
use common::*;
pub fn IsFeatureImplemented<T: Tracer>(
    state: &mut State,
    tracer: &T,
    feat: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        feat: u32,
    }
    let fn_state = FunctionState {
        feat,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var feat:u32
        let s_0_0: u32 = fn_state.feat;
        // D s_0_1: call num_of_Feature(s_0_0)
        let s_0_1: i64 = num_of_Feature(state, tracer, s_0_0);
        // C s_0_2: const #102872u : u32
        let s_0_2: u32 = 102872;
        // D s_0_3: read-reg s_0_2:[u8; 259]
        let s_0_3: [bool; 259usize] = {
            let value = state.read_register::<[bool; 259usize]>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_1 -> i
        let s_0_4: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_5: read-element s_0_3[s_0_4]
        let s_0_5: bool = s_0_3[(s_0_4) as usize];
        // N s_0_6: return s_0_5
        return s_0_5;
    }
}
