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
use Bit::*;
use AArch64_NextRandomTagBit::*;
use common::*;
pub fn AArch64_RandomTag<T: Tracer>(state: &mut State, tracer: &T, gs_15780: ()) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        tag: u8,
        i: i64,
        gs_15780: (),
    }
    let fn_state = FunctionState {
        gs_15780,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #0s : i64
        let s_0_0: i64 = 0;
        // D s_0_1: write-var i <= s_0_0
        fn_state.i = s_0_0;
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #3s : i64
        let s_1_1: i64 = 3;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call AArch64_NextRandomTagBit(s_2_0)
        let s_2_1: bool = AArch64_NextRandomTagBit(state, tracer, s_2_0);
        // S s_2_2: call Bit(s_2_1)
        let s_2_2: bool = Bit(state, tracer, s_2_1);
        // D s_2_3: read-var tag:u8
        let s_2_3: u8 = fn_state.tag;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 4u16);
        // D s_2_5: read-var i:i64
        let s_2_5: i64 = fn_state.i;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_7: const #1u : u64
        let s_2_7: u64 = 1;
        // D s_2_8: bit-insert s_2_4 s_2_4 s_2_6 s_2_7
        let s_2_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_4.length(),
            );
            (s_2_4 & mask) | (s_2_4 << s_2_6)
        };
        // D s_2_9: cast reint s_2_8 -> u8
        let s_2_9: u8 = (s_2_8.value() as u8);
        // D s_2_10: write-var tag <= s_2_9
        fn_state.tag = s_2_9;
        // D s_2_11: read-var i:i64
        let s_2_11: i64 = fn_state.i;
        // C s_2_12: const #1s : i64
        let s_2_12: i64 = 1;
        // D s_2_13: add s_2_11 s_2_12
        let s_2_13: i64 = (s_2_11 + s_2_12);
        // D s_2_14: write-var i <= s_2_13
        fn_state.i = s_2_13;
        // N s_2_15: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var tag:u8
        let s_3_0: u8 = fn_state.tag;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
}
