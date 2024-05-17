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
use CurrentInstrSet::*;
use common::*;
pub fn InITBlock<T: Tracer>(state: &mut State, tracer: &T, gs_31560: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        gs_31560: (),
    }
    let fn_state = FunctionState {
        gs_31560,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CurrentInstrSet(s_0_0)
        let s_0_1: u32 = CurrentInstrSet(state, tracer, s_0_0);
        // C s_0_2: const #2u : u32
        let s_0_2: u32 = 2;
        // S s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
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
        // D s_1_1: write-var return_value <= s_1_0
        fn_state.return_value = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var return_value:u8
        let s_2_0: bool = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #16981u : u32
        let s_3_0: u32 = 16981;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: cast zx s_3_1 -> bv
        let s_3_3: Bits = Bits::new(s_3_1 as u128, 8u16);
        // C s_3_4: const #1s : i64
        let s_3_4: i64 = 1;
        // C s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // C s_3_6: const #3s : i
        let s_3_6: i128 = 3;
        // C s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: bit-extract s_3_3 s_3_2 s_3_7
        let s_3_8: Bits = (Bits::new(
            ((s_3_3) >> (s_3_2)).value(),
            u16::try_from(s_3_7).unwrap(),
        ));
        // D s_3_9: cast reint s_3_8 -> u8
        let s_3_9: u8 = (s_3_8.value() as u8);
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 4u16);
        // C s_3_11: const #0u : u8
        let s_3_11: u8 = 0;
        // C s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 4u16);
        // D s_3_13: cmp-ne s_3_10 s_3_12
        let s_3_13: bool = ((s_3_10) != (s_3_12));
        // D s_3_14: write-var return_value <= s_3_13
        fn_state.return_value = s_3_13;
        // N s_3_15: jump b2
        return block_2(state, tracer, fn_state);
    }
}
