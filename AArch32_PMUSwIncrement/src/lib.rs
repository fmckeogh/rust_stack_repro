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
use neq_int::*;
use AArch32_GetNumEventCountersAccessible::*;
use u_get_PMCR_Type_E::*;
use u__id::*;
use PMUEvent__2::*;
use u_get_HDCR_Type_HPME::*;
use PMCR_read::*;
use HDCR_read::*;
use common::*;
pub fn AArch32_PMUSwIncrement<T: Tracer>(
    state: &mut State,
    tracer: &T,
    sw_incr: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_32218: bool,
        idx: i64,
        gs_32213: i64,
        gs_32205: bool,
        counters: i128,
        sw_incr: u32,
    }
    let fn_state = FunctionState {
        sw_incr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call PMCR_read(s_0_0)
        let s_0_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_0_0);
        // S s_0_2: call _get_PMCR_Type_E(s_0_1)
        let s_0_2: bool = u_get_PMCR_Type_E(state, tracer, s_0_1);
        // S s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // S s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b16 b1
        if s_0_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#32205 <= s_1_0
        fn_state.gs_32205 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#32205:u8
        let s_2_0: bool = fn_state.gs_32205;
        // N s_2_1: branch s_2_0 b15 b3
        if s_2_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call AArch32_GetNumEventCountersAccessible(s_3_0)
        let s_3_1: i128 = AArch32_GetNumEventCountersAccessible(state, tracer, s_3_0);
        // D s_3_2: write-var counters <= s_3_1
        fn_state.counters = s_3_1;
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // D s_3_4: read-var counters:i
        let s_3_4: i128 = fn_state.counters;
        // D s_3_5: call neq_int(s_3_4, s_3_3)
        let s_3_5: bool = neq_int(state, tracer, s_3_4, s_3_3);
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var counters:i
        let s_5_2: i128 = fn_state.counters;
        // D s_5_3: sub s_5_2 s_5_1
        let s_5_3: i128 = ((s_5_2) - (s_5_1));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var gs#32213 <= s_5_4
        fn_state.gs_32213 = s_5_4;
        // D s_5_6: write-var idx <= s_5_0
        fn_state.idx = s_5_0;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var idx:i64
        let s_6_0: i64 = fn_state.idx;
        // D s_6_1: read-var gs#32213:i64
        let s_6_1: i64 = fn_state.gs_32213;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b14 b7
        if s_6_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var idx:i64
        let s_7_0: i64 = fn_state.idx;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // C s_7_3: const #0s : i
        let s_7_3: i128 = 0;
        // D s_7_4: cmp-le s_7_3 s_7_2
        let s_7_4: bool = ((s_7_3) <= (s_7_2));
        // N s_7_5: branch s_7_4 b13 b8
        if s_7_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#32218 <= s_8_0
        fn_state.gs_32218 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#32218:u8
        let s_9_0: bool = fn_state.gs_32218;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var sw_incr:u32
        let s_9_2: u32 = fn_state.sw_incr;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 32u16);
        // D s_9_4: read-var idx:i64
        let s_9_4: i64 = fn_state.idx;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // C s_9_6: const #1u : u64
        let s_9_6: u64 = 1;
        // D s_9_7: bit-extract s_9_3 s_9_5 s_9_6
        let s_9_7: Bits = (Bits::new(
            ((s_9_3) >> (s_9_5)).value(),
            u16::try_from(s_9_6).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u8
        let s_9_8: bool = ((s_9_7.value()) != 0);
        // C s_9_9: const #0s : i
        let s_9_9: i128 = 0;
        // C s_9_10: const #0u : u64
        let s_9_10: u64 = 0;
        // D s_9_11: cast zx s_9_8 -> u64
        let s_9_11: u64 = (s_9_8 as u64);
        // C s_9_12: const #1u : u64
        let s_9_12: u64 = 1;
        // D s_9_13: and s_9_11 s_9_12
        let s_9_13: u64 = ((s_9_11) & (s_9_12));
        // D s_9_14: cmp-eq s_9_13 s_9_12
        let s_9_14: bool = ((s_9_13) == (s_9_12));
        // D s_9_15: lsl s_9_11 s_9_9
        let s_9_15: u64 = s_9_11 << s_9_9;
        // D s_9_16: or s_9_10 s_9_15
        let s_9_16: u64 = ((s_9_10) | (s_9_15));
        // D s_9_17: cmpl s_9_15
        let s_9_17: u64 = !s_9_15;
        // D s_9_18: and s_9_10 s_9_17
        let s_9_18: u64 = ((s_9_10) & (s_9_17));
        // D s_9_19: select s_9_14 s_9_16 s_9_18
        let s_9_19: u64 = if s_9_14 { s_9_16 } else { s_9_18 };
        // D s_9_20: cast trunc s_9_19 -> u8
        let s_9_20: bool = ((s_9_19) != 0);
        // D s_9_21: cast zx s_9_20 -> bv
        let s_9_21: Bits = Bits::new(s_9_20 as u128, 1u16);
        // C s_9_22: const #1u : u8
        let s_9_22: bool = true;
        // C s_9_23: cast zx s_9_22 -> bv
        let s_9_23: Bits = Bits::new(s_9_22 as u128, 1u16);
        // D s_9_24: cmp-eq s_9_21 s_9_23
        let s_9_24: bool = ((s_9_21) == (s_9_23));
        // N s_9_25: branch s_9_24 b12 b10
        if s_9_24 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var idx:i64
        let s_11_0: i64 = fn_state.idx;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var idx <= s_11_2
        fn_state.idx = s_11_2;
        // N s_11_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1s : i
        let s_12_0: i128 = 1;
        // D s_12_1: read-var idx:i64
        let s_12_1: i64 = fn_state.idx;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // C s_12_3: const #16u : u32
        let s_12_3: u32 = 16;
        // D s_12_4: read-reg s_12_3:u16
        let s_12_4: u16 = {
            let value = state.read_register::<u16>(s_12_3 as isize);
            tracer.read_register(s_12_3 as isize, value);
            value
        };
        // D s_12_5: call PMUEvent__2(s_12_4, s_12_0, s_12_2)
        let s_12_5: () = PMUEvent__2(state, tracer, s_12_4, s_12_0, s_12_2);
        // N s_12_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var idx:i64
        let s_13_0: i64 = fn_state.idx;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call __id(s_13_1)
        let s_13_2: i128 = u__id(state, tracer, s_13_1);
        // C s_13_3: const #32s : i
        let s_13_3: i128 = 32;
        // D s_13_4: cmp-lt s_13_2 s_13_3
        let s_13_4: bool = ((s_13_2) < (s_13_3));
        // D s_13_5: write-var gs#32218 <= s_13_4
        fn_state.gs_32218 = s_13_4;
        // N s_13_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call HDCR_read(s_16_0)
        let s_16_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_16_0);
        // S s_16_2: call _get_HDCR_Type_HPME(s_16_1)
        let s_16_2: bool = u_get_HDCR_Type_HPME(state, tracer, s_16_1);
        // S s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #0u : u8
        let s_16_4: bool = false;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // S s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // D s_16_7: write-var gs#32205 <= s_16_6
        fn_state.gs_32205 = s_16_6;
        // N s_16_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
