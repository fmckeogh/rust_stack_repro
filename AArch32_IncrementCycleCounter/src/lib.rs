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
use u_update_PMOVSR_Type_C::*;
use Mk_PMCCNTR_Type::*;
use u_get_PMCR_Type_LC::*;
use HasElapsed64Cycles::*;
use PMCCNTR_read::*;
use PMOVSR_write::*;
use PMOVSSET_read::*;
use PMOVSSET_write::*;
use CountPMUEvents::*;
use PMCR_read::*;
use integer_subrange::*;
use u_update_PMOVSSET_Type_C::*;
use subrange_subrange_eq::*;
use u_get_PMCR_Type_D::*;
use PMOVSR_read::*;
use PMCCNTR_write::*;
use common::*;
pub fn AArch32_IncrementCycleCounter<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_32239: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_25249: ProductType5c790c8ef59cc8b2,
        gs_32240: bool,
        gs_32242: bool,
        old_value: i128,
        ga_25258: i64,
        gs_32241: bool,
        new_value: i128,
        gs_32239: (),
    }
    let fn_state = FunctionState {
        gs_32239,
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
        // N s_0_3: branch s_0_2 b10 b1
        if s_0_2 {
            return block_10(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#32242 <= s_1_0
        fn_state.gs_32242 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#32242:u8
        let s_2_0: bool = fn_state.gs_32242;
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
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call PMCCNTR_read(s_4_0)
        let s_4_1: ProductType5c790c8ef59cc8b2 = PMCCNTR_read(state, tracer, s_4_0);
        // D s_4_2: write-var ga#25249 <= s_4_1
        fn_state.ga_25249 = s_4_1;
        // D s_4_3: read-var ga#25249.0:struct
        let s_4_3: u64 = fn_state.ga_25249._0;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 64u16);
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (s_4_4.value() as i128);
        // D s_4_6: write-var old_value <= s_4_5
        fn_state.old_value = s_4_5;
        // C s_4_7: const #1s : i
        let s_4_7: i128 = 1;
        // D s_4_8: read-var old_value:i
        let s_4_8: i128 = fn_state.old_value;
        // D s_4_9: add s_4_8 s_4_7
        let s_4_9: i128 = (s_4_8 + s_4_7);
        // D s_4_10: write-var new_value <= s_4_9
        fn_state.new_value = s_4_9;
        // C s_4_11: const #63s : i
        let s_4_11: i128 = 63;
        // C s_4_12: const #0s : i
        let s_4_12: i128 = 0;
        // D s_4_13: read-var new_value:i
        let s_4_13: i128 = fn_state.new_value;
        // D s_4_14: call integer_subrange(s_4_13, s_4_11, s_4_12)
        let s_4_14: Bits = integer_subrange(state, tracer, s_4_13, s_4_11, s_4_12);
        // D s_4_15: cast reint s_4_14 -> u64
        let s_4_15: u64 = (s_4_14.value() as u64);
        // D s_4_16: call Mk_PMCCNTR_Type(s_4_15)
        let s_4_16: ProductType5c790c8ef59cc8b2 = Mk_PMCCNTR_Type(state, tracer, s_4_15);
        // D s_4_17: call PMCCNTR_write(s_4_16)
        let s_4_17: () = PMCCNTR_write(state, tracer, s_4_16);
        // C s_4_18: const #() : ()
        let s_4_18: () = ();
        // S s_4_19: call PMCR_read(s_4_18)
        let s_4_19: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_4_18);
        // S s_4_20: call _get_PMCR_Type_LC(s_4_19)
        let s_4_20: bool = u_get_PMCR_Type_LC(state, tracer, s_4_19);
        // S s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 1u16);
        // C s_4_22: const #1u : u8
        let s_4_22: bool = true;
        // C s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 1u16);
        // S s_4_24: cmp-eq s_4_21 s_4_23
        let s_4_24: bool = ((s_4_21) == (s_4_23));
        // N s_4_25: branch s_4_24 b9 b5
        if s_4_24 {
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
        // C s_5_0: const #32s : i64
        let s_5_0: i64 = 32;
        // D s_5_1: write-var ga#25258 <= s_5_0
        fn_state.ga_25258 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#25258:i64
        let s_6_0: i64 = fn_state.ga_25258;
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
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call PMOVSSET_read(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = PMOVSSET_read(state, tracer, s_8_0);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // S s_8_3: call _update_PMOVSSET_Type_C(s_8_1, s_8_2)
        let s_8_3: ProductType700c18a878c5601b = u_update_PMOVSSET_Type_C(
            state,
            tracer,
            s_8_1,
            s_8_2,
        );
        // S s_8_4: call PMOVSSET_write(s_8_3)
        let s_8_4: () = PMOVSSET_write(state, tracer, s_8_3);
        // C s_8_5: const #() : ()
        let s_8_5: () = ();
        // S s_8_6: call PMOVSR_read(s_8_5)
        let s_8_6: ProductType700c18a878c5601b = PMOVSR_read(state, tracer, s_8_5);
        // C s_8_7: const #1u : u8
        let s_8_7: bool = true;
        // S s_8_8: call _update_PMOVSR_Type_C(s_8_6, s_8_7)
        let s_8_8: ProductType700c18a878c5601b = u_update_PMOVSR_Type_C(
            state,
            tracer,
            s_8_6,
            s_8_7,
        );
        // S s_8_9: call PMOVSR_write(s_8_8)
        let s_8_9: () = PMOVSR_write(state, tracer, s_8_8);
        // N s_8_10: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // D s_9_1: write-var ga#25258 <= s_9_0
        fn_state.ga_25258 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call PMCR_read(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_10_0);
        // S s_10_2: call _get_PMCR_Type_LC(s_10_1)
        let s_10_2: bool = u_get_PMCR_Type_LC(state, tracer, s_10_1);
        // S s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // S s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // N s_10_7: branch s_10_6 b16 b11
        if s_10_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call PMCR_read(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_11_0);
        // S s_11_2: call _get_PMCR_Type_D(s_11_1)
        let s_11_2: bool = u_get_PMCR_Type_D(state, tracer, s_11_1);
        // S s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #0u : u8
        let s_11_4: bool = false;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // S s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // D s_11_7: write-var gs#32240 <= s_11_6
        fn_state.gs_32240 = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#32240:u8
        let s_12_0: bool = fn_state.gs_32240;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HasElapsed64Cycles(s_13_0)
        let s_13_1: bool = HasElapsed64Cycles(state, tracer, s_13_0);
        // D s_13_2: write-var gs#32241 <= s_13_1
        fn_state.gs_32241 = s_13_1;
        // N s_13_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#32241:u8
        let s_14_0: bool = fn_state.gs_32241;
        // D s_14_1: write-var gs#32242 <= s_14_0
        fn_state.gs_32242 = s_14_0;
        // N s_14_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#32241 <= s_15_0
        fn_state.gs_32241 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#32240 <= s_16_0
        fn_state.gs_32240 = s_16_0;
        // N s_16_2: jump b12
        return block_12(state, tracer, fn_state);
    }
}
