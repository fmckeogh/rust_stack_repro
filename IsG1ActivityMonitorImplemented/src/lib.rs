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
pub fn IsG1ActivityMonitorImplemented<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_331523: bool,
        i: i128,
    }
    let fn_state = FunctionState {
        i,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var i:i
        let s_0_1: i128 = fn_state.i;
        // D s_0_2: cmp-le s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) <= (s_0_1));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#331523 <= s_1_0
        fn_state.gs_331523 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#331523:u8
        let s_2_0: bool = fn_state.gs_331523;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #104520u : u32
        let s_2_2: u32 = 104520;
        // D s_2_3: read-reg s_2_2:u16
        let s_2_3: u16 = {
            let value = state.read_register::<u16>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 16u16);
        // D s_2_5: read-var i:i
        let s_2_5: i128 = fn_state.i;
        // C s_2_6: const #1u : u64
        let s_2_6: u64 = 1;
        // D s_2_7: bit-extract s_2_4 s_2_5 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_4) >> (s_2_5)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: bool = ((s_2_7.value()) != 0);
        // C s_2_9: const #1u : u8
        let s_2_9: bool = true;
        // D s_2_10: cmp-eq s_2_8 s_2_9
        let s_2_10: bool = ((s_2_8) == (s_2_9));
        // N s_2_11: return s_2_10
        return s_2_10;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // D s_3_1: read-var i:i
        let s_3_1: i128 = fn_state.i;
        // D s_3_2: cmp-lt s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) < (s_3_0));
        // D s_3_3: write-var gs#331523 <= s_3_2
        fn_state.gs_331523 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
