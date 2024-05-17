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
use u__IMPDEF_bits::*;
use AArch64_HaveS1TG::*;
use common::*;
pub fn AArch64_S1DecodeTG0<T: Tracer>(state: &mut State, tracer: &T, tg0_in: u8) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        ga_10112: u8,
        tgx: u32,
        tg0: u8,
        tg0_in: u8,
    }
    let fn_state = FunctionState {
        tg0_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_0_0: read-var tg0_in:u8
        let s_0_0: u8 = fn_state.tg0_in;
        // D s_0_1: write-var tg0 <= s_0_0
        fn_state.tg0 = s_0_0;
        // D s_0_2: read-var tg0:u8
        let s_0_2: u8 = fn_state.tg0;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // C s_0_4: const #3u : u8
        let s_0_4: u8 = 3;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b20 b1
        if s_0_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var tg0:u8
        let s_2_0: u8 = fn_state.tg0;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: not s_2_4
        let s_2_5: bool = !s_2_4;
        // N s_2_6: branch s_2_5 b15 b3
        if s_2_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: write-var tgx <= s_3_0
        fn_state.tgx = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var tgx:u32
        let s_4_0: u32 = fn_state.tgx;
        // D s_4_1: call AArch64_HaveS1TG(s_4_0)
        let s_4_1: bool = AArch64_HaveS1TG(state, tracer, s_4_0);
        // D s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var tgx:u32
        let s_6_0: u32 = fn_state.tgx;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #2s : i64
        let s_7_0: i64 = 2;
        // C s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // C s_7_2: const #"TG0 encoded granule size" : str
        let s_7_2: &'static str = "TG0 encoded granule size";
        // S s_7_3: call __IMPDEF_bits(s_7_1, s_7_2)
        let s_7_3: Bits = u__IMPDEF_bits(state, tracer, s_7_1, s_7_2);
        // S s_7_4: cast reint s_7_3 -> u8
        let s_7_4: u8 = (s_7_3.value() as u8);
        // D s_7_5: write-var ga#10112 <= s_7_4
        fn_state.ga_10112 = s_7_4;
        // D s_7_6: read-var ga#10112:u8
        let s_7_6: u8 = fn_state.ga_10112;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 2u16);
        // C s_7_8: const #0u : u8
        let s_7_8: u8 = 0;
        // C s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 2u16);
        // D s_7_10: cmp-eq s_7_7 s_7_9
        let s_7_10: bool = ((s_7_7) == (s_7_9));
        // D s_7_11: not s_7_10
        let s_7_11: bool = !s_7_10;
        // N s_7_12: branch s_7_11 b10 b8
        if s_7_11 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_8_0: const #0u : u32
        let s_8_0: u32 = 0;
        // D s_8_1: write-var tgx <= s_8_0
        fn_state.tgx = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_9_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_10_0: read-var ga#10112:u8
        let s_10_0: u8 = fn_state.ga_10112;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #1u : u8
        let s_10_2: u8 = 1;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_11_0: const #2u : u32
        let s_11_0: u32 = 2;
        // D s_11_1: write-var tgx <= s_11_0
        fn_state.tgx = s_11_0;
        // N s_11_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_12_0: read-var ga#10112:u8
        let s_12_0: u8 = fn_state.ga_10112;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #2u : u8
        let s_12_2: u8 = 2;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_13_0: const #1u : u32
        let s_13_0: u32 = 1;
        // D s_13_1: write-var tgx <= s_13_0
        fn_state.tgx = s_13_0;
        // N s_13_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_14_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_15_0: read-var tg0:u8
        let s_15_0: u8 = fn_state.tg0;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #1u : u8
        let s_15_2: u8 = 1;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b17 b16
        if s_15_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_16_0: const #2u : u32
        let s_16_0: u32 = 2;
        // D s_16_1: write-var tgx <= s_16_0
        fn_state.tgx = s_16_0;
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_17_0: read-var tg0:u8
        let s_17_0: u8 = fn_state.tg0;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_18_0: const #1u : u32
        let s_18_0: u32 = 1;
        // D s_18_1: write-var tgx <= s_18_0
        fn_state.tgx = s_18_0;
        // N s_18_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_19_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_20_0: const #2s : i64
        let s_20_0: i64 = 2;
        // C s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // C s_20_2: const #"TG0 encoded granule size" : str
        let s_20_2: &'static str = "TG0 encoded granule size";
        // S s_20_3: call __IMPDEF_bits(s_20_1, s_20_2)
        let s_20_3: Bits = u__IMPDEF_bits(state, tracer, s_20_1, s_20_2);
        // S s_20_4: cast reint s_20_3 -> u8
        let s_20_4: u8 = (s_20_3.value() as u8);
        // D s_20_5: write-var tg0 <= s_20_4
        fn_state.tg0 = s_20_4;
        // N s_20_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
