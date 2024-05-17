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
use AArch64_IncrementEventCounter::*;
use GetNumEventCounters::*;
use PMUCountValue::*;
use Mk_PMOVSR_Type::*;
use PMOVSR_write::*;
use PMOVSSET_read::*;
use PMUEvent__2::*;
use PMEVCNTR_set::*;
use PMOVSSET_write::*;
use u__id::*;
use integer_subrange::*;
use HaveAArch64::*;
use subrange_subrange_eq::*;
use Mk_PMOVSSET_Type::*;
use integer_access::*;
use PMEVCNTR_read::*;
use PMOVSR_read::*;
use common::*;
pub fn AArch32_IncrementEventCounter<T: Tracer>(
    state: &mut State,
    tracer: &T,
    idx: i64,
    increment_name: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_25199: ProductType700c18a878c5601b,
        ga_25195: ProductType700c18a878c5601b,
        gs_32177: bool,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveAArch64(s_0_0)
        let s_0_1: bool = HaveAArch64(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b9 b1
        if s_0_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var idx:i64
        let s_1_0: i64 = fn_state.idx;
        // D s_1_1: call PMEVCNTR_read(s_1_0)
        let s_1_1: u32 = PMEVCNTR_read(state, tracer, s_1_0);
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 32u16);
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (s_1_2.value() as i128);
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: read-var idx:i64
        let s_1_5: i64 = fn_state.idx;
        // D s_1_6: read-var increment_name:i
        let s_1_6: i128 = fn_state.increment_name;
        // D s_1_7: call PMUCountValue(s_1_5, s_1_6)
        let s_1_7: i128 = PMUCountValue(state, tracer, s_1_5, s_1_6);
        // D s_1_8: cast zx s_1_4 -> i
        let s_1_8: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_9: add s_1_8 s_1_7
        let s_1_9: i128 = (s_1_8 + s_1_7);
        // C s_1_10: const #31s : i
        let s_1_10: i128 = 31;
        // C s_1_11: const #0s : i
        let s_1_11: i128 = 0;
        // D s_1_12: call integer_subrange(s_1_9, s_1_10, s_1_11)
        let s_1_12: Bits = integer_subrange(state, tracer, s_1_9, s_1_10, s_1_11);
        // D s_1_13: cast reint s_1_12 -> u32
        let s_1_13: u32 = (s_1_12.value() as u32);
        // D s_1_14: read-var idx:i64
        let s_1_14: i64 = fn_state.idx;
        // D s_1_15: call PMEVCNTR_set(s_1_14, s_1_13)
        let s_1_15: () = PMEVCNTR_set(state, tracer, s_1_14, s_1_13);
        // C s_1_16: const #32s : i64
        let s_1_16: i64 = 32;
        // C s_1_17: const #64s : i
        let s_1_17: i128 = 64;
        // C s_1_18: const #0s : i
        let s_1_18: i128 = 0;
        // D s_1_19: cast zx s_1_4 -> i
        let s_1_19: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_20: call integer_subrange(s_1_19, s_1_17, s_1_18)
        let s_1_20: Bits = integer_subrange(state, tracer, s_1_19, s_1_17, s_1_18);
        // D s_1_21: cast reint s_1_20 -> u65
        let s_1_21: u128 = (s_1_20.value() as u128);
        // C s_1_22: const #64s : i
        let s_1_22: i128 = 64;
        // C s_1_23: const #0s : i
        let s_1_23: i128 = 0;
        // D s_1_24: call integer_subrange(s_1_9, s_1_22, s_1_23)
        let s_1_24: Bits = integer_subrange(state, tracer, s_1_9, s_1_22, s_1_23);
        // D s_1_25: cast reint s_1_24 -> u65
        let s_1_25: u128 = (s_1_24.value() as u128);
        // C s_1_26: const #64s : i
        let s_1_26: i128 = 64;
        // C s_1_27: const #64s : i
        let s_1_27: i128 = 64;
        // D s_1_28: cast zx s_1_21 -> bv
        let s_1_28: Bits = Bits::new(s_1_21 as u128, 65u16);
        // C s_1_29: cast zx s_1_16 -> i
        let s_1_29: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_30: cast zx s_1_25 -> bv
        let s_1_30: Bits = Bits::new(s_1_25 as u128, 65u16);
        // C s_1_31: cast zx s_1_16 -> i
        let s_1_31: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_32: call subrange_subrange_eq(s_1_28, s_1_26, s_1_29, s_1_30, s_1_27, s_1_31)
        let s_1_32: bool = subrange_subrange_eq(
            state,
            tracer,
            s_1_28,
            s_1_26,
            s_1_29,
            s_1_30,
            s_1_27,
            s_1_31,
        );
        // D s_1_33: not s_1_32
        let s_1_33: bool = !s_1_32;
        // N s_1_34: branch s_1_33 b3 b2
        if s_1_33 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call PMOVSSET_read(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = PMOVSSET_read(state, tracer, s_3_0);
        // D s_3_2: write-var ga#25195 <= s_3_1
        fn_state.ga_25195 = s_3_1;
        // D s_3_3: read-var ga#25195.0:struct
        let s_3_3: u32 = fn_state.ga_25195._0;
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 32u16);
        // D s_3_5: read-var idx:i64
        let s_3_5: i64 = fn_state.idx;
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: read-var idx:i64
        let s_3_7: i64 = fn_state.idx;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // C s_3_9: const #1u : u8
        let s_3_9: bool = true;
        // C s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 1u16);
        // D s_3_11: sub s_3_6 s_3_8
        let s_3_11: i128 = ((s_3_6) - (s_3_8));
        // C s_3_12: const #1u : u64
        let s_3_12: u64 = 1;
        // C s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 64u16);
        // D s_3_14: lsl s_3_13 s_3_11
        let s_3_14: Bits = s_3_13 << s_3_11;
        // D s_3_15: sub s_3_14 s_3_13
        let s_3_15: Bits = ((s_3_14) - (s_3_13));
        // D s_3_16: and s_3_10 s_3_15
        let s_3_16: Bits = ((s_3_10) & (s_3_15));
        // D s_3_17: lsl s_3_16 s_3_8
        let s_3_17: Bits = s_3_16 << s_3_8;
        // D s_3_18: lsl s_3_15 s_3_8
        let s_3_18: Bits = s_3_15 << s_3_8;
        // D s_3_19: cmpl s_3_18
        let s_3_19: Bits = !s_3_18;
        // D s_3_20: and s_3_4 s_3_19
        let s_3_20: Bits = ((s_3_4) & (s_3_19));
        // D s_3_21: or s_3_20 s_3_17
        let s_3_21: Bits = ((s_3_20) | (s_3_17));
        // D s_3_22: cast reint s_3_21 -> u32
        let s_3_22: u32 = (s_3_21.value() as u32);
        // D s_3_23: call Mk_PMOVSSET_Type(s_3_22)
        let s_3_23: ProductType700c18a878c5601b = Mk_PMOVSSET_Type(
            state,
            tracer,
            s_3_22,
        );
        // D s_3_24: call PMOVSSET_write(s_3_23)
        let s_3_24: () = PMOVSSET_write(state, tracer, s_3_23);
        // C s_3_25: const #() : ()
        let s_3_25: () = ();
        // S s_3_26: call PMOVSR_read(s_3_25)
        let s_3_26: ProductType700c18a878c5601b = PMOVSR_read(state, tracer, s_3_25);
        // D s_3_27: write-var ga#25199 <= s_3_26
        fn_state.ga_25199 = s_3_26;
        // D s_3_28: read-var ga#25199.0:struct
        let s_3_28: u32 = fn_state.ga_25199._0;
        // D s_3_29: cast zx s_3_28 -> bv
        let s_3_29: Bits = Bits::new(s_3_28 as u128, 32u16);
        // D s_3_30: read-var idx:i64
        let s_3_30: i64 = fn_state.idx;
        // D s_3_31: cast zx s_3_30 -> i
        let s_3_31: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_32: read-var idx:i64
        let s_3_32: i64 = fn_state.idx;
        // D s_3_33: cast zx s_3_32 -> i
        let s_3_33: i128 = (i128::try_from(s_3_32).unwrap());
        // C s_3_34: const #1u : u8
        let s_3_34: bool = true;
        // C s_3_35: cast zx s_3_34 -> bv
        let s_3_35: Bits = Bits::new(s_3_34 as u128, 1u16);
        // D s_3_36: sub s_3_31 s_3_33
        let s_3_36: i128 = ((s_3_31) - (s_3_33));
        // C s_3_37: const #1u : u64
        let s_3_37: u64 = 1;
        // C s_3_38: cast zx s_3_37 -> bv
        let s_3_38: Bits = Bits::new(s_3_37 as u128, 64u16);
        // D s_3_39: lsl s_3_38 s_3_36
        let s_3_39: Bits = s_3_38 << s_3_36;
        // D s_3_40: sub s_3_39 s_3_38
        let s_3_40: Bits = ((s_3_39) - (s_3_38));
        // D s_3_41: and s_3_35 s_3_40
        let s_3_41: Bits = ((s_3_35) & (s_3_40));
        // D s_3_42: lsl s_3_41 s_3_33
        let s_3_42: Bits = s_3_41 << s_3_33;
        // D s_3_43: lsl s_3_40 s_3_33
        let s_3_43: Bits = s_3_40 << s_3_33;
        // D s_3_44: cmpl s_3_43
        let s_3_44: Bits = !s_3_43;
        // D s_3_45: and s_3_29 s_3_44
        let s_3_45: Bits = ((s_3_29) & (s_3_44));
        // D s_3_46: or s_3_45 s_3_42
        let s_3_46: Bits = ((s_3_45) | (s_3_42));
        // D s_3_47: cast reint s_3_46 -> u32
        let s_3_47: u32 = (s_3_46.value() as u32);
        // D s_3_48: call Mk_PMOVSR_Type(s_3_47)
        let s_3_48: ProductType700c18a878c5601b = Mk_PMOVSR_Type(state, tracer, s_3_47);
        // D s_3_49: call PMOVSR_write(s_3_48)
        let s_3_49: () = PMOVSR_write(state, tracer, s_3_48);
        // C s_3_50: const #0s : i
        let s_3_50: i128 = 0;
        // D s_3_51: read-var idx:i64
        let s_3_51: i64 = fn_state.idx;
        // D s_3_52: cast zx s_3_51 -> i
        let s_3_52: i128 = (i128::try_from(s_3_51).unwrap());
        // D s_3_53: call integer_access(s_3_52, s_3_50)
        let s_3_53: bool = integer_access(state, tracer, s_3_52, s_3_50);
        // C s_3_54: const #0s : i
        let s_3_54: i128 = 0;
        // C s_3_55: const #0u : u64
        let s_3_55: u64 = 0;
        // D s_3_56: cast zx s_3_53 -> u64
        let s_3_56: u64 = (s_3_53 as u64);
        // C s_3_57: const #1u : u64
        let s_3_57: u64 = 1;
        // D s_3_58: and s_3_56 s_3_57
        let s_3_58: u64 = ((s_3_56) & (s_3_57));
        // D s_3_59: cmp-eq s_3_58 s_3_57
        let s_3_59: bool = ((s_3_58) == (s_3_57));
        // D s_3_60: lsl s_3_56 s_3_54
        let s_3_60: u64 = s_3_56 << s_3_54;
        // D s_3_61: or s_3_55 s_3_60
        let s_3_61: u64 = ((s_3_55) | (s_3_60));
        // D s_3_62: cmpl s_3_60
        let s_3_62: u64 = !s_3_60;
        // D s_3_63: and s_3_55 s_3_62
        let s_3_63: u64 = ((s_3_55) & (s_3_62));
        // D s_3_64: select s_3_59 s_3_61 s_3_63
        let s_3_64: u64 = if s_3_59 { s_3_61 } else { s_3_63 };
        // D s_3_65: cast trunc s_3_64 -> u8
        let s_3_65: bool = ((s_3_64) != 0);
        // D s_3_66: cast zx s_3_65 -> bv
        let s_3_66: Bits = Bits::new(s_3_65 as u128, 1u16);
        // C s_3_67: const #0u : u8
        let s_3_67: bool = false;
        // C s_3_68: cast zx s_3_67 -> bv
        let s_3_68: Bits = Bits::new(s_3_67 as u128, 1u16);
        // D s_3_69: cmp-eq s_3_66 s_3_68
        let s_3_69: bool = ((s_3_66) == (s_3_68));
        // N s_3_70: branch s_3_69 b8 b4
        if s_3_69 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#32177 <= s_4_0
        fn_state.gs_32177 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#32177:u8
        let s_5_0: bool = fn_state.gs_32177;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var idx:i64
        let s_7_1: i64 = fn_state.idx;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: add s_7_2 s_7_0
        let s_7_3: i128 = (s_7_2 + s_7_0);
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // C s_7_5: const #1s : i
        let s_7_5: i128 = 1;
        // D s_7_6: cast zx s_7_4 -> i
        let s_7_6: i128 = (i128::try_from(s_7_4).unwrap());
        // C s_7_7: const #80u : u32
        let s_7_7: u32 = 80;
        // D s_7_8: read-reg s_7_7:u16
        let s_7_8: u16 = {
            let value = state.read_register::<u16>(s_7_7 as isize);
            tracer.read_register(s_7_7 as isize, value);
            value
        };
        // D s_7_9: call PMUEvent__2(s_7_8, s_7_5, s_7_6)
        let s_7_9: () = PMUEvent__2(state, tracer, s_7_8, s_7_5, s_7_6);
        // N s_7_10: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i
        let s_8_0: i128 = 1;
        // D s_8_1: read-var idx:i64
        let s_8_1: i64 = fn_state.idx;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: add s_8_2 s_8_0
        let s_8_3: i128 = (s_8_2 + s_8_0);
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // C s_8_5: const #() : ()
        let s_8_5: () = ();
        // S s_8_6: call GetNumEventCounters(s_8_5)
        let s_8_6: i128 = GetNumEventCounters(state, tracer, s_8_5);
        // D s_8_7: cast zx s_8_4 -> i
        let s_8_7: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_8: cmp-lt s_8_7 s_8_6
        let s_8_8: bool = ((s_8_7) < (s_8_6));
        // D s_8_9: write-var gs#32177 <= s_8_8
        fn_state.gs_32177 = s_8_8;
        // N s_8_10: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var idx:i64
        let s_9_0: i64 = fn_state.idx;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #0s : i
        let s_9_4: i128 = 0;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-le s_9_4 s_9_5
        let s_9_6: bool = ((s_9_4) <= (s_9_5));
        // N s_9_7: assert s_9_6
        let s_9_7: () = assert!(s_9_6);
        // D s_9_8: read-var idx:i64
        let s_9_8: i64 = fn_state.idx;
        // D s_9_9: cast zx s_9_8 -> i
        let s_9_9: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_10: call __id(s_9_9)
        let s_9_10: i128 = u__id(state, tracer, s_9_9);
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // C s_9_12: const #31s : i
        let s_9_12: i128 = 31;
        // D s_9_13: cast zx s_9_11 -> i
        let s_9_13: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_14: cmp-lt s_9_13 s_9_12
        let s_9_14: bool = ((s_9_13) < (s_9_12));
        // N s_9_15: assert s_9_14
        let s_9_15: () = assert!(s_9_14);
        // D s_9_16: read-var idx:i64
        let s_9_16: i64 = fn_state.idx;
        // D s_9_17: read-var increment_name:i
        let s_9_17: i128 = fn_state.increment_name;
        // D s_9_18: call AArch64_IncrementEventCounter(s_9_16, s_9_17)
        let s_9_18: () = AArch64_IncrementEventCounter(state, tracer, s_9_16, s_9_17);
        // N s_9_19: return
        return;
    }
}
