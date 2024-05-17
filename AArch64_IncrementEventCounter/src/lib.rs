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
use PMUEvent__2::*;
use PMUCounterIsHyp::*;
use GetNumEventCounters::*;
use u_get_PMCR_EL0_Type_LP::*;
use PMUCountValue::*;
use HavePMUv3p5::*;
use integer_subrange::*;
use u_get_MDCR_EL2_Type_HLP::*;
use subrange_subrange_eq::*;
use Bit::*;
use integer_access::*;
use common::*;
pub fn AArch64_IncrementEventCounter<T: Tracer>(
    state: &mut State,
    tracer: &T,
    idx: i64,
    increment_name: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        lp: bool,
        gs_25385: bool,
        ovflw: i64,
        old_value: i128,
        gs_25386: bool,
        new_value: i128,
        gs_25387: bool,
        idx: i64,
        increment_name: i128,
    }
    let fn_state = FunctionState {
        idx,
        increment_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #32s : i64
        let s_0_0: i64 = 32;
        // D s_0_1: write-var ovflw <= s_0_0
        fn_state.ovflw = s_0_0;
        // C s_0_2: const #10744u : u32
        let s_0_2: u32 = 10744;
        // D s_0_3: read-reg s_0_2:[u64; 32]
        let s_0_3: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: read-var idx:i64
        let s_0_4: i64 = fn_state.idx;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: read-element s_0_3[s_0_5]
        let s_0_6: u64 = s_0_3[(s_0_5) as usize];
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 64u16);
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (s_0_7.value() as i128);
        // D s_0_9: write-var old_value <= s_0_8
        fn_state.old_value = s_0_8;
        // D s_0_10: read-var idx:i64
        let s_0_10: i64 = fn_state.idx;
        // D s_0_11: read-var increment_name:i
        let s_0_11: i128 = fn_state.increment_name;
        // D s_0_12: call PMUCountValue(s_0_10, s_0_11)
        let s_0_12: i128 = PMUCountValue(state, tracer, s_0_10, s_0_11);
        // D s_0_13: read-var old_value:i
        let s_0_13: i128 = fn_state.old_value;
        // D s_0_14: add s_0_13 s_0_12
        let s_0_14: i128 = (s_0_13 + s_0_12);
        // D s_0_15: write-var new_value <= s_0_14
        fn_state.new_value = s_0_14;
        // C s_0_16: const #() : ()
        let s_0_16: () = ();
        // S s_0_17: call HavePMUv3p5(s_0_16)
        let s_0_17: bool = HavePMUv3p5(state, tracer, s_0_16);
        // N s_0_18: branch s_0_17 b16 b1
        if s_0_17 {
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
        // C s_1_0: const #31s : i
        let s_1_0: i128 = 31;
        // C s_1_1: const #0s : i
        let s_1_1: i128 = 0;
        // D s_1_2: read-var new_value:i
        let s_1_2: i128 = fn_state.new_value;
        // D s_1_3: call integer_subrange(s_1_2, s_1_0, s_1_1)
        let s_1_3: Bits = integer_subrange(state, tracer, s_1_2, s_1_0, s_1_1);
        // D s_1_4: cast reint s_1_3 -> u32
        let s_1_4: u32 = (s_1_3.value() as u32);
        // C s_1_5: const #64s : i
        let s_1_5: i128 = 64;
        // D s_1_6: cast zx s_1_4 -> bv
        let s_1_6: Bits = Bits::new(s_1_4 as u128, 32u16);
        // D s_1_7: bits-cast zx s_1_6 -> bv length s_1_5
        let s_1_7: Bits = s_1_6.zero_extend(s_1_5);
        // D s_1_8: cast reint s_1_7 -> u64
        let s_1_8: u64 = (s_1_7.value() as u64);
        // C s_1_9: const #10744u : u32
        let s_1_9: u32 = 10744;
        // D s_1_10: read-reg s_1_9:[u64; 32]
        let s_1_10: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_1_9 as isize);
            tracer.read_register(s_1_9 as isize, value);
            value
        };
        // D s_1_11: read-var idx:i64
        let s_1_11: i64 = fn_state.idx;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: mutate-element s_1_10[s_1_12] <= s_1_8
        let s_1_13: [u64; 32usize] = {
            let mut local = s_1_10.clone();
            local[(s_1_12) as usize] = s_1_8;
            local
        };
        // D s_1_14: cast cvt s_1_13 -> [u64; 0]
        let s_1_14: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_1_13);
        // D s_1_15: cast cvt s_1_14 -> [u64; 32]
        let s_1_15: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_1_14);
            buf
        };
        // C s_1_16: const #10744u : u32
        let s_1_16: u32 = 10744;
        // N s_1_17: write-reg s_1_16 <= s_1_15
        let s_1_17: () = {
            state.write_register::<[u64; 32usize]>(s_1_16 as isize, s_1_15);
            tracer.write_register(s_1_16 as isize, s_1_15);
        };
        // C s_1_18: const #32s : i64
        let s_1_18: i64 = 32;
        // D s_1_19: write-var ovflw <= s_1_18
        fn_state.ovflw = s_1_18;
        // N s_1_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ovflw:i64
        let s_2_0: i64 = fn_state.ovflw;
        // C s_2_1: const #64s : i
        let s_2_1: i128 = 64;
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: read-var old_value:i
        let s_2_3: i128 = fn_state.old_value;
        // D s_2_4: call integer_subrange(s_2_3, s_2_1, s_2_2)
        let s_2_4: Bits = integer_subrange(state, tracer, s_2_3, s_2_1, s_2_2);
        // D s_2_5: cast reint s_2_4 -> u65
        let s_2_5: u128 = (s_2_4.value() as u128);
        // C s_2_6: const #64s : i
        let s_2_6: i128 = 64;
        // C s_2_7: const #0s : i
        let s_2_7: i128 = 0;
        // D s_2_8: read-var new_value:i
        let s_2_8: i128 = fn_state.new_value;
        // D s_2_9: call integer_subrange(s_2_8, s_2_6, s_2_7)
        let s_2_9: Bits = integer_subrange(state, tracer, s_2_8, s_2_6, s_2_7);
        // D s_2_10: cast reint s_2_9 -> u65
        let s_2_10: u128 = (s_2_9.value() as u128);
        // C s_2_11: const #64s : i
        let s_2_11: i128 = 64;
        // C s_2_12: const #64s : i
        let s_2_12: i128 = 64;
        // D s_2_13: cast zx s_2_5 -> bv
        let s_2_13: Bits = Bits::new(s_2_5 as u128, 65u16);
        // D s_2_14: cast zx s_2_0 -> i
        let s_2_14: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_15: cast zx s_2_10 -> bv
        let s_2_15: Bits = Bits::new(s_2_10 as u128, 65u16);
        // D s_2_16: cast zx s_2_0 -> i
        let s_2_16: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_17: call subrange_subrange_eq(s_2_13, s_2_11, s_2_14, s_2_15, s_2_12, s_2_16)
        let s_2_17: bool = subrange_subrange_eq(
            state,
            tracer,
            s_2_13,
            s_2_11,
            s_2_14,
            s_2_15,
            s_2_12,
            s_2_16,
        );
        // D s_2_18: not s_2_17
        let s_2_18: bool = !s_2_17;
        // N s_2_19: branch s_2_18 b4 b3
        if s_2_18 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // S s_4_1: call Bit(s_4_0)
        let s_4_1: bool = Bit(state, tracer, s_4_0);
        // C s_4_2: const #104640u : u32
        let s_4_2: u32 = 104640;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // C s_4_4: const #104640u : u32
        let s_4_4: u32 = 104640;
        // N s_4_5: write-reg s_4_4 <= s_4_3
        let s_4_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_4 as isize, s_4_3);
            tracer.write_register(s_4_4 as isize, s_4_3);
        };
        // C s_4_6: const #1u : u8
        let s_4_6: bool = true;
        // S s_4_7: call Bit(s_4_6)
        let s_4_7: bool = Bit(state, tracer, s_4_6);
        // C s_4_8: const #104888u : u32
        let s_4_8: u32 = 104888;
        // D s_4_9: read-reg s_4_8:struct
        let s_4_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_8 as isize);
            tracer.read_register(s_4_8 as isize, value);
            value
        };
        // C s_4_10: const #104888u : u32
        let s_4_10: u32 = 104888;
        // N s_4_11: write-reg s_4_10 <= s_4_9
        let s_4_11: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_10 as isize, s_4_9);
            tracer.write_register(s_4_10 as isize, s_4_9);
        };
        // C s_4_12: const #0s : i
        let s_4_12: i128 = 0;
        // D s_4_13: read-var idx:i64
        let s_4_13: i64 = fn_state.idx;
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: call integer_access(s_4_14, s_4_12)
        let s_4_15: bool = integer_access(state, tracer, s_4_14, s_4_12);
        // C s_4_16: const #0s : i
        let s_4_16: i128 = 0;
        // C s_4_17: const #0u : u64
        let s_4_17: u64 = 0;
        // D s_4_18: cast zx s_4_15 -> u64
        let s_4_18: u64 = (s_4_15 as u64);
        // C s_4_19: const #1u : u64
        let s_4_19: u64 = 1;
        // D s_4_20: and s_4_18 s_4_19
        let s_4_20: u64 = ((s_4_18) & (s_4_19));
        // D s_4_21: cmp-eq s_4_20 s_4_19
        let s_4_21: bool = ((s_4_20) == (s_4_19));
        // D s_4_22: lsl s_4_18 s_4_16
        let s_4_22: u64 = s_4_18 << s_4_16;
        // D s_4_23: or s_4_17 s_4_22
        let s_4_23: u64 = ((s_4_17) | (s_4_22));
        // D s_4_24: cmpl s_4_22
        let s_4_24: u64 = !s_4_22;
        // D s_4_25: and s_4_17 s_4_24
        let s_4_25: u64 = ((s_4_17) & (s_4_24));
        // D s_4_26: select s_4_21 s_4_23 s_4_25
        let s_4_26: u64 = if s_4_21 { s_4_23 } else { s_4_25 };
        // D s_4_27: cast trunc s_4_26 -> u8
        let s_4_27: bool = ((s_4_26) != 0);
        // D s_4_28: cast zx s_4_27 -> bv
        let s_4_28: Bits = Bits::new(s_4_27 as u128, 1u16);
        // C s_4_29: const #0u : u8
        let s_4_29: bool = false;
        // C s_4_30: cast zx s_4_29 -> bv
        let s_4_30: Bits = Bits::new(s_4_29 as u128, 1u16);
        // D s_4_31: cmp-eq s_4_28 s_4_30
        let s_4_31: bool = ((s_4_28) == (s_4_30));
        // N s_4_32: branch s_4_31 b15 b5
        if s_4_31 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#25385 <= s_5_0
        fn_state.gs_25385 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#25385:u8
        let s_6_0: bool = fn_state.gs_25385;
        // N s_6_1: branch s_6_0 b11 b7
        if s_6_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#25387 <= s_7_0
        fn_state.gs_25387 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#25387:u8
        let s_8_0: bool = fn_state.gs_25387;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i
        let s_10_0: i128 = 1;
        // D s_10_1: read-var idx:i64
        let s_10_1: i64 = fn_state.idx;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: add s_10_2 s_10_0
        let s_10_3: i128 = (s_10_2 + s_10_0);
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // C s_10_5: const #1s : i
        let s_10_5: i128 = 1;
        // D s_10_6: cast zx s_10_4 -> i
        let s_10_6: i128 = (i128::try_from(s_10_4).unwrap());
        // C s_10_7: const #80u : u32
        let s_10_7: u32 = 80;
        // D s_10_8: read-reg s_10_7:u16
        let s_10_8: u16 = {
            let value = state.read_register::<u16>(s_10_7 as isize);
            tracer.read_register(s_10_7 as isize, value);
            value
        };
        // D s_10_9: call PMUEvent__2(s_10_8, s_10_5, s_10_6)
        let s_10_9: () = PMUEvent__2(state, tracer, s_10_8, s_10_5, s_10_6);
        // N s_10_10: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HavePMUv3p5(s_11_0)
        let s_11_1: bool = HavePMUv3p5(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // N s_11_3: branch s_11_2 b14 b12
        if s_11_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var lp:u8
        let s_12_0: bool = fn_state.lp;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #0u : u8
        let s_12_2: bool = false;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#25386 <= s_12_4
        fn_state.gs_25386 = s_12_4;
        // N s_12_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#25386:u8
        let s_13_0: bool = fn_state.gs_25386;
        // D s_13_1: write-var gs#25387 <= s_13_0
        fn_state.gs_25387 = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#25386 <= s_14_0
        fn_state.gs_25386 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1s : i
        let s_15_0: i128 = 1;
        // D s_15_1: read-var idx:i64
        let s_15_1: i64 = fn_state.idx;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: add s_15_2 s_15_0
        let s_15_3: i128 = (s_15_2 + s_15_0);
        // D s_15_4: cast reint s_15_3 -> i64
        let s_15_4: i64 = (s_15_3 as i64);
        // C s_15_5: const #() : ()
        let s_15_5: () = ();
        // S s_15_6: call GetNumEventCounters(s_15_5)
        let s_15_6: i128 = GetNumEventCounters(state, tracer, s_15_5);
        // D s_15_7: cast zx s_15_4 -> i
        let s_15_7: i128 = (i128::try_from(s_15_4).unwrap());
        // D s_15_8: cmp-lt s_15_7 s_15_6
        let s_15_8: bool = ((s_15_7) < (s_15_6));
        // D s_15_9: write-var gs#25385 <= s_15_8
        fn_state.gs_25385 = s_15_8;
        // N s_15_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #63s : i
        let s_16_0: i128 = 63;
        // C s_16_1: const #0s : i
        let s_16_1: i128 = 0;
        // D s_16_2: read-var new_value:i
        let s_16_2: i128 = fn_state.new_value;
        // D s_16_3: call integer_subrange(s_16_2, s_16_0, s_16_1)
        let s_16_3: Bits = integer_subrange(state, tracer, s_16_2, s_16_0, s_16_1);
        // D s_16_4: cast reint s_16_3 -> u64
        let s_16_4: u64 = (s_16_3.value() as u64);
        // C s_16_5: const #10744u : u32
        let s_16_5: u32 = 10744;
        // D s_16_6: read-reg s_16_5:[u64; 32]
        let s_16_6: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_16_5 as isize);
            tracer.read_register(s_16_5 as isize, value);
            value
        };
        // D s_16_7: read-var idx:i64
        let s_16_7: i64 = fn_state.idx;
        // D s_16_8: cast zx s_16_7 -> i
        let s_16_8: i128 = (i128::try_from(s_16_7).unwrap());
        // D s_16_9: mutate-element s_16_6[s_16_8] <= s_16_4
        let s_16_9: [u64; 32usize] = {
            let mut local = s_16_6.clone();
            local[(s_16_8) as usize] = s_16_4;
            local
        };
        // D s_16_10: cast cvt s_16_9 -> [u64; 0]
        let s_16_10: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_16_9);
        // D s_16_11: cast cvt s_16_10 -> [u64; 32]
        let s_16_11: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_16_10);
            buf
        };
        // C s_16_12: const #10744u : u32
        let s_16_12: u32 = 10744;
        // N s_16_13: write-reg s_16_12 <= s_16_11
        let s_16_13: () = {
            state.write_register::<[u64; 32usize]>(s_16_12 as isize, s_16_11);
            tracer.write_register(s_16_12 as isize, s_16_11);
        };
        // D s_16_14: read-var idx:i64
        let s_16_14: i64 = fn_state.idx;
        // D s_16_15: cast zx s_16_14 -> i
        let s_16_15: i128 = (i128::try_from(s_16_14).unwrap());
        // D s_16_16: call PMUCounterIsHyp(s_16_15)
        let s_16_16: bool = PMUCounterIsHyp(state, tracer, s_16_15);
        // N s_16_17: branch s_16_16 b22 b17
        if s_16_16 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #21016u : u32
        let s_17_0: u32 = 21016;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_PMCR_EL0_Type_LP(s_17_1)
        let s_17_2: bool = u_get_PMCR_EL0_Type_LP(state, tracer, s_17_1);
        // D s_17_3: write-var lp <= s_17_2
        fn_state.lp = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var lp:u8
        let s_18_0: bool = fn_state.lp;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b21 b19
        if s_18_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #32s : i64
        let s_19_0: i64 = 32;
        // D s_19_1: write-var ovflw <= s_19_0
        fn_state.ovflw = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // D s_21_1: write-var ovflw <= s_21_0
        fn_state.ovflw = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #104880u : u32
        let s_22_0: u32 = 104880;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_MDCR_EL2_Type_HLP(s_22_1)
        let s_22_2: bool = u_get_MDCR_EL2_Type_HLP(state, tracer, s_22_1);
        // D s_22_3: write-var lp <= s_22_2
        fn_state.lp = s_22_2;
        // N s_22_4: jump b18
        return block_18(state, tracer, fn_state);
    }
}
