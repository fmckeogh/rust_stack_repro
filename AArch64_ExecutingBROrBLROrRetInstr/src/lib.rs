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
use ThisInstr::*;
use HaveBTIExt::*;
use common::*;
pub fn AArch64_ExecutingBROrBLROrRetInstr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_6504: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_6510: bool,
        instr: u32,
        return_value: bool,
        gs_6504: (),
    }
    let fn_state = FunctionState {
        gs_6504,
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
        // S s_0_1: call HaveBTIExt(s_0_0)
        let s_0_1: bool = HaveBTIExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b8 b1
        if s_0_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call ThisInstr(s_1_0)
        let s_1_1: u32 = ThisInstr(state, tracer, s_1_0);
        // D s_1_2: write-var instr <= s_1_1
        fn_state.instr = s_1_1;
        // C s_1_3: const #25s : i
        let s_1_3: i128 = 25;
        // D s_1_4: read-var instr:u32
        let s_1_4: u32 = fn_state.instr;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 32u16);
        // C s_1_6: const #1s : i64
        let s_1_6: i64 = 1;
        // C s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // C s_1_8: const #6s : i
        let s_1_8: i128 = 6;
        // C s_1_9: add s_1_8 s_1_7
        let s_1_9: i128 = (s_1_8 + s_1_7);
        // D s_1_10: bit-extract s_1_5 s_1_3 s_1_9
        let s_1_10: Bits = (Bits::new(
            ((s_1_5) >> (s_1_3)).value(),
            u16::try_from(s_1_9).unwrap(),
        ));
        // D s_1_11: cast reint s_1_10 -> u8
        let s_1_11: u8 = (s_1_10.value() as u8);
        // D s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 7u16);
        // C s_1_13: const #107u : u8
        let s_1_13: u8 = 107;
        // C s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 7u16);
        // D s_1_15: cmp-eq s_1_12 s_1_14
        let s_1_15: bool = ((s_1_12) == (s_1_14));
        // N s_1_16: branch s_1_15 b7 b2
        if s_1_15 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#6510 <= s_2_0
        fn_state.gs_6510 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#6510:u8
        let s_3_0: bool = fn_state.gs_6510;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var return_value:u8
        let s_5_0: bool = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #21s : i
        let s_6_0: i128 = 21;
        // D s_6_1: read-var instr:u32
        let s_6_1: u32 = fn_state.instr;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // C s_6_3: const #1s : i64
        let s_6_3: i64 = 1;
        // C s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // C s_6_5: const #3s : i
        let s_6_5: i128 = 3;
        // C s_6_6: add s_6_5 s_6_4
        let s_6_6: i128 = (s_6_5 + s_6_4);
        // D s_6_7: bit-extract s_6_2 s_6_0 s_6_6
        let s_6_7: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_6).unwrap(),
        ));
        // D s_6_8: cast reint s_6_7 -> u8
        let s_6_8: u8 = (s_6_7.value() as u8);
        // D s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 4u16);
        // C s_6_10: const #5u : u8
        let s_6_10: u8 = 5;
        // C s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 4u16);
        // D s_6_12: cmp-ne s_6_9 s_6_11
        let s_6_12: bool = ((s_6_9) != (s_6_11));
        // D s_6_13: write-var return_value <= s_6_12
        fn_state.return_value = s_6_12;
        // N s_6_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #16s : i
        let s_7_0: i128 = 16;
        // D s_7_1: read-var instr:u32
        let s_7_1: u32 = fn_state.instr;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 32u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #4s : i
        let s_7_5: i128 = 4;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_0 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 5u16);
        // C s_7_10: const #31u : u8
        let s_7_10: u8 = 31;
        // C s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 5u16);
        // D s_7_12: cmp-eq s_7_9 s_7_11
        let s_7_12: bool = ((s_7_9) == (s_7_11));
        // D s_7_13: write-var gs#6510 <= s_7_12
        fn_state.gs_6510 = s_7_12;
        // N s_7_14: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
