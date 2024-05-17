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
use AArch32_S1IASize::*;
use is_zero_subrange::*;
use u__UNKNOWN_VARange::*;
use is_ones_subrange::*;
use common::*;
pub fn AArch32_GetVARange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u32,
    t0sz: u8,
    t1sz: u8,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_27436: bool,
        lo_iasize: i128,
        ga_21126: u32,
        return_value: u32,
        up_iasize: i128,
        ga_21129: u32,
        va: u32,
        t0sz: u8,
        t1sz: u8,
    }
    let fn_state = FunctionState {
        va,
        t0sz,
        t1sz,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_0_0: read-var t0sz:u8
        let s_0_0: u8 = fn_state.t0sz;
        // D s_0_1: call AArch32_S1IASize(s_0_0)
        let s_0_1: i128 = AArch32_S1IASize(state, tracer, s_0_0);
        // D s_0_2: write-var lo_iasize <= s_0_1
        fn_state.lo_iasize = s_0_1;
        // D s_0_3: read-var t1sz:u8
        let s_0_3: u8 = fn_state.t1sz;
        // D s_0_4: call AArch32_S1IASize(s_0_3)
        let s_0_4: i128 = AArch32_S1IASize(state, tracer, s_0_3);
        // D s_0_5: write-var up_iasize <= s_0_4
        fn_state.up_iasize = s_0_4;
        // D s_0_6: read-var t1sz:u8
        let s_0_6: u8 = fn_state.t1sz;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 3u16);
        // C s_0_8: const #0u : u8
        let s_0_8: u8 = 0;
        // C s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 3u16);
        // D s_0_10: cmp-eq s_0_7 s_0_9
        let s_0_10: bool = ((s_0_7) == (s_0_9));
        // N s_0_11: branch s_0_10 b20 b1
        if s_0_10 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#27436 <= s_1_0
        fn_state.gs_27436 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#27436:u8
        let s_2_0: bool = fn_state.gs_27436;
        // N s_2_1: branch s_2_0 b19 b3
        if s_2_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_3_0: read-var t1sz:u8
        let s_3_0: u8 = fn_state.t1sz;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 3u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 3u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b15 b4
        if s_3_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var t0sz:u8
        let s_4_0: u8 = fn_state.t0sz;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 3u16);
        // C s_4_2: const #0u : u8
        let s_4_2: u8 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 3u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b11 b5
        if s_4_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: read-var va:u32
        let s_5_1: u32 = fn_state.va;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 32u16);
        // D s_5_3: read-var lo_iasize:i
        let s_5_3: i128 = fn_state.lo_iasize;
        // D s_5_4: call is_zero_subrange(s_5_2, s_5_0, s_5_3)
        let s_5_4: bool = is_zero_subrange(state, tracer, s_5_2, s_5_0, s_5_3);
        // N s_5_5: branch s_5_4 b10 b6
        if s_5_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_6_0: const #31s : i
        let s_6_0: i128 = 31;
        // D s_6_1: read-var va:u32
        let s_6_1: u32 = fn_state.va;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_3: read-var up_iasize:i
        let s_6_3: i128 = fn_state.up_iasize;
        // D s_6_4: call is_ones_subrange(s_6_2, s_6_0, s_6_3)
        let s_6_4: bool = is_ones_subrange(state, tracer, s_6_2, s_6_0, s_6_3);
        // N s_6_5: branch s_6_4 b9 b7
        if s_6_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call __UNKNOWN_VARange(s_7_0)
        let s_7_1: u32 = u__UNKNOWN_VARange(state, tracer, s_7_0);
        // D s_7_2: write-var return_value <= s_7_1
        fn_state.return_value = s_7_1;
        // N s_7_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var return_value:u32
        let s_8_0: u32 = fn_state.return_value;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #1u : u32
        let s_9_0: u32 = 1;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_10_0: const #0u : u32
        let s_10_0: u32 = 0;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_11_0: const #31s : i
        let s_11_0: i128 = 31;
        // D s_11_1: read-var va:u32
        let s_11_1: u32 = fn_state.va;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 32u16);
        // D s_11_3: read-var up_iasize:i
        let s_11_3: i128 = fn_state.up_iasize;
        // D s_11_4: call is_ones_subrange(s_11_2, s_11_0, s_11_3)
        let s_11_4: bool = is_ones_subrange(state, tracer, s_11_2, s_11_0, s_11_3);
        // N s_11_5: branch s_11_4 b14 b12
        if s_11_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #0u : u32
        let s_12_0: u32 = 0;
        // D s_12_1: write-var ga#21129 <= s_12_0
        fn_state.ga_21129 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_13_0: read-var ga#21129:u32
        let s_13_0: u32 = fn_state.ga_21129;
        // D s_13_1: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // D s_14_1: write-var ga#21129 <= s_14_0
        fn_state.ga_21129 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_15_0: const #31s : i
        let s_15_0: i128 = 31;
        // D s_15_1: read-var va:u32
        let s_15_1: u32 = fn_state.va;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 32u16);
        // D s_15_3: read-var lo_iasize:i
        let s_15_3: i128 = fn_state.lo_iasize;
        // D s_15_4: call is_zero_subrange(s_15_2, s_15_0, s_15_3)
        let s_15_4: bool = is_zero_subrange(state, tracer, s_15_2, s_15_0, s_15_3);
        // N s_15_5: branch s_15_4 b18 b16
        if s_15_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_16_0: const #1u : u32
        let s_16_0: u32 = 1;
        // D s_16_1: write-var ga#21126 <= s_16_0
        fn_state.ga_21126 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_17_0: read-var ga#21126:u32
        let s_17_0: u32 = fn_state.ga_21126;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_18_0: const #0u : u32
        let s_18_0: u32 = 0;
        // D s_18_1: write-var ga#21126 <= s_18_0
        fn_state.ga_21126 = s_18_0;
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_19_0: const #0u : u32
        let s_19_0: u32 = 0;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_20_0: read-var t0sz:u8
        let s_20_0: u8 = fn_state.t0sz;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 3u16);
        // C s_20_2: const #0u : u8
        let s_20_2: u8 = 0;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 3u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: write-var gs#27436 <= s_20_4
        fn_state.gs_27436 = s_20_4;
        // N s_20_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
