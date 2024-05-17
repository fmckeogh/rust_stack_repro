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
use common::*;
pub fn AMEVTYPER1_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_0_0: const #13296u : u32
        let s_0_0: u32 = 13296;
        // D s_0_1: read-reg s_0_0:[struct; 16]
        let s_0_1: [ProductType700c18a878c5601b; 16usize] = {
            let value = state
                .read_register::<[ProductType700c18a878c5601b; 16usize]>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: read-var n:i64
        let s_0_2: i64 = fn_state.n;
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: read-element s_0_1[s_0_3]
        let s_0_4: ProductType700c18a878c5601b = s_0_1[(s_0_3) as usize];
        // N s_0_5: return s_0_4
        return s_0_4;
    }
}
