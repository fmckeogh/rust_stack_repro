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
use u_get_PMCR_EL0_Type_LC::*;
use CountPMUEvents::*;
use integer_subrange::*;
use HaveAArch32::*;
use Mk_PMCCNTR_EL0_Type::*;
use subrange_subrange_eq::*;
use HasElapsed64Cycles::*;
use u_get_PMCR_EL0_Type_D::*;
use common::*;
pub fn AArch64_IncrementCycleCounter<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25433: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25437: bool,
        ovflw: i64,
        old_value: i128,
        gs_25436: bool,
        gs_25434: bool,
        new_value: i128,
        gs_25435: bool,
        gs_25433: (),
    }
    let fn_state = FunctionState {
        gs_25433,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: read-reg s_0_0:i64
        let s_0_1: i64 = {
            let value = state.read_register::<i64>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call CountPMUEvents(s_0_1)
        let s_0_2: bool = CountPMUEvents(state, tracer, s_0_1);
        // N s_0_3: branch s_0_2 b13 b1
        if s_0_2 {
            return block_13(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#25437 <= s_1_0
        fn_state.gs_25437 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25437:u8
        let s_2_0: bool = fn_state.gs_25437;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
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
        // C s_4_0: const #104776u : u32
        let s_4_0: u32 = 104776;
        // D s_4_1: read-reg s_4_0:u64
        let s_4_1: u64 = {
            let value = state.read_register::<u64>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (s_4_2.value() as i128);
        // D s_4_4: write-var old_value <= s_4_3
        fn_state.old_value = s_4_3;
        // C s_4_5: const #1s : i
        let s_4_5: i128 = 1;
        // D s_4_6: read-var old_value:i
        let s_4_6: i128 = fn_state.old_value;
        // D s_4_7: add s_4_6 s_4_5
        let s_4_7: i128 = (s_4_6 + s_4_5);
        // D s_4_8: write-var new_value <= s_4_7
        fn_state.new_value = s_4_7;
        // C s_4_9: const #63s : i
        let s_4_9: i128 = 63;
        // C s_4_10: const #0s : i
        let s_4_10: i128 = 0;
        // D s_4_11: read-var new_value:i
        let s_4_11: i128 = fn_state.new_value;
        // D s_4_12: call integer_subrange(s_4_11, s_4_9, s_4_10)
        let s_4_12: Bits = integer_subrange(state, tracer, s_4_11, s_4_9, s_4_10);
        // D s_4_13: cast reint s_4_12 -> u64
        let s_4_13: u64 = (s_4_12.value() as u64);
        // D s_4_14: call Mk_PMCCNTR_EL0_Type(s_4_13)
        let s_4_14: ProductType5c790c8ef59cc8b2 = Mk_PMCCNTR_EL0_Type(
            state,
            tracer,
            s_4_13,
        );
        // C s_4_15: const #104776u : u32
        let s_4_15: u32 = 104776;
        // N s_4_16: write-reg s_4_15 <= s_4_14
        let s_4_16: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_15 as isize, s_4_14);
            tracer.write_register(s_4_15 as isize, s_4_14);
        };
        // C s_4_17: const #32s : i64
        let s_4_17: i64 = 32;
        // D s_4_18: write-var ovflw <= s_4_17
        fn_state.ovflw = s_4_17;
        // C s_4_19: const #() : ()
        let s_4_19: () = ();
        // S s_4_20: call HaveAArch32(s_4_19)
        let s_4_20: bool = HaveAArch32(state, tracer, s_4_19);
        // N s_4_21: branch s_4_20 b9 b5
        if s_4_20 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: write-var ovflw <= s_5_0
        fn_state.ovflw = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ovflw:i64
        let s_6_0: i64 = fn_state.ovflw;
        // C s_6_1: const #64s : i
        let s_6_1: i128 = 64;
        // C s_6_2: const #0s : i
        let s_6_2: i128 = 0;
        // D s_6_3: read-var old_value:i
        let s_6_3: i128 = fn_state.old_value;
        // D s_6_4: call integer_subrange(s_6_3, s_6_1, s_6_2)
        let s_6_4: Bits = integer_subrange(state, tracer, s_6_3, s_6_1, s_6_2);
        // D s_6_5: cast reint s_6_4 -> u65
        let s_6_5: u128 = (s_6_4.value() as u128);
        // C s_6_6: const #64s : i
        let s_6_6: i128 = 64;
        // C s_6_7: const #0s : i
        let s_6_7: i128 = 0;
        // D s_6_8: read-var new_value:i
        let s_6_8: i128 = fn_state.new_value;
        // D s_6_9: call integer_subrange(s_6_8, s_6_6, s_6_7)
        let s_6_9: Bits = integer_subrange(state, tracer, s_6_8, s_6_6, s_6_7);
        // D s_6_10: cast reint s_6_9 -> u65
        let s_6_10: u128 = (s_6_9.value() as u128);
        // C s_6_11: const #64s : i
        let s_6_11: i128 = 64;
        // C s_6_12: const #64s : i
        let s_6_12: i128 = 64;
        // D s_6_13: cast zx s_6_5 -> bv
        let s_6_13: Bits = Bits::new(s_6_5 as u128, 65u16);
        // D s_6_14: cast zx s_6_0 -> i
        let s_6_14: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_15: cast zx s_6_10 -> bv
        let s_6_15: Bits = Bits::new(s_6_10 as u128, 65u16);
        // D s_6_16: cast zx s_6_0 -> i
        let s_6_16: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_17: call subrange_subrange_eq(s_6_13, s_6_11, s_6_14, s_6_15, s_6_12, s_6_16)
        let s_6_17: bool = subrange_subrange_eq(
            state,
            tracer,
            s_6_13,
            s_6_11,
            s_6_14,
            s_6_15,
            s_6_12,
            s_6_16,
        );
        // D s_6_18: not s_6_17
        let s_6_18: bool = !s_6_17;
        // N s_6_19: branch s_6_18 b8 b7
        if s_6_18 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #104640u : u32
        let s_8_0: u32 = 104640;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #104640u : u32
        let s_8_2: u32 = 104640;
        // N s_8_3: write-reg s_8_2 <= s_8_1
        let s_8_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_2 as isize, s_8_1);
            tracer.write_register(s_8_2 as isize, s_8_1);
        };
        // C s_8_4: const #104888u : u32
        let s_8_4: u32 = 104888;
        // D s_8_5: read-reg s_8_4:struct
        let s_8_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_4 as isize);
            tracer.read_register(s_8_4 as isize, value);
            value
        };
        // C s_8_6: const #104888u : u32
        let s_8_6: u32 = 104888;
        // N s_8_7: write-reg s_8_6 <= s_8_5
        let s_8_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_6 as isize, s_8_5);
            tracer.write_register(s_8_6 as isize, s_8_5);
        };
        // N s_8_8: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #21016u : u32
        let s_9_0: u32 = 21016;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_PMCR_EL0_Type_LC(s_9_1)
        let s_9_2: bool = u_get_PMCR_EL0_Type_LC(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // N s_9_7: branch s_9_6 b12 b10
        if s_9_6 {
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
        // C s_10_0: const #32s : i64
        let s_10_0: i64 = 32;
        // D s_10_1: write-var ovflw <= s_10_0
        fn_state.ovflw = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // D s_12_1: write-var ovflw <= s_12_0
        fn_state.ovflw = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HaveAArch32(s_13_0)
        let s_13_1: bool = HaveAArch32(state, tracer, s_13_0);
        // S s_13_2: not s_13_1
        let s_13_2: bool = !s_13_1;
        // N s_13_3: branch s_13_2 b22 b14
        if s_13_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #21016u : u32
        let s_14_0: u32 = 21016;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_PMCR_EL0_Type_LC(s_14_1)
        let s_14_2: bool = u_get_PMCR_EL0_Type_LC(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // D s_14_7: write-var gs#25434 <= s_14_6
        fn_state.gs_25434 = s_14_6;
        // N s_14_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#25434:u8
        let s_15_0: bool = fn_state.gs_25434;
        // N s_15_1: branch s_15_0 b21 b16
        if s_15_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #21016u : u32
        let s_16_0: u32 = 21016;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_PMCR_EL0_Type_D(s_16_1)
        let s_16_2: bool = u_get_PMCR_EL0_Type_D(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #0u : u8
        let s_16_4: bool = false;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // D s_16_7: write-var gs#25435 <= s_16_6
        fn_state.gs_25435 = s_16_6;
        // N s_16_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#25435:u8
        let s_17_0: bool = fn_state.gs_25435;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HasElapsed64Cycles(s_18_0)
        let s_18_1: bool = HasElapsed64Cycles(state, tracer, s_18_0);
        // D s_18_2: write-var gs#25436 <= s_18_1
        fn_state.gs_25436 = s_18_1;
        // N s_18_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#25436:u8
        let s_19_0: bool = fn_state.gs_25436;
        // D s_19_1: write-var gs#25437 <= s_19_0
        fn_state.gs_25437 = s_19_0;
        // N s_19_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#25436 <= s_20_0
        fn_state.gs_25436 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#25435 <= s_21_0
        fn_state.gs_25435 = s_21_0;
        // N s_21_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#25434 <= s_22_0
        fn_state.gs_25434 = s_22_0;
        // N s_22_2: jump b15
        return block_15(state, tracer, fn_state);
    }
}
