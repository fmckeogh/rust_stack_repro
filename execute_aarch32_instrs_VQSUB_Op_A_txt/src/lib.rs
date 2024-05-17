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
use SatQ::*;
use Elem_set::*;
use u_update_FPSCR_Type_QC::*;
use CheckAdvSIMDEnabled::*;
use D_set::*;
use asl_Int::*;
use FPSCR_read__1::*;
use Elem_read::*;
use FPSCR_write::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VQSUB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    regs: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        esizeshadow_7740: i64,
        ga_359784: ProductTypef506aa96a892fe52,
        d: i128,
        gs_317408: i64,
        gs_317402: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        regs: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        n,
        regs,
        is_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#7740 <= s_0_2
        fn_state.esizeshadow_7740 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#317402 <= s_1_5
        fn_state.gs_317402 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#317402:i64
        let s_2_1: i64 = fn_state.gs_317402;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#317408 <= s_3_4
        fn_state.gs_317408 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#317408:i64
        let s_4_1: i64 = fn_state.gs_317408;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
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
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call D_read(s_5_6)
        let s_5_7: u64 = D_read(state, tracer, s_5_6);
        // D s_5_8: read-var esizeshadow#7740:i64
        let s_5_8: i64 = fn_state.esizeshadow_7740;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_7 -> bv
        let s_5_11: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: call Elem_read(s_5_11, s_5_13, s_5_14)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_11, s_5_13, s_5_14);
        // D s_5_16: read-var is_unsigned:u8
        let s_5_16: bool = fn_state.is_unsigned;
        // D s_5_17: call asl_Int(s_5_15, s_5_16)
        let s_5_17: i128 = asl_Int(state, tracer, s_5_15, s_5_16);
        // D s_5_18: read-var m:i64
        let s_5_18: i64 = fn_state.m;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: read-var r:i64
        let s_5_20: i64 = fn_state.r;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: add s_5_19 s_5_21
        let s_5_22: i128 = (s_5_19 + s_5_21);
        // D s_5_23: cast reint s_5_22 -> i64
        let s_5_23: i64 = (s_5_22 as i64);
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: call D_read(s_5_24)
        let s_5_25: u64 = D_read(state, tracer, s_5_24);
        // D s_5_26: read-var esizeshadow#7740:i64
        let s_5_26: i64 = fn_state.esizeshadow_7740;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: cast zx s_5_25 -> bv
        let s_5_29: Bits = Bits::new(s_5_25 as u128, 64u16);
        // D s_5_30: read-var e:i64
        let s_5_30: i64 = fn_state.e;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: cast zx s_5_28 -> i
        let s_5_32: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_33: call Elem_read(s_5_29, s_5_31, s_5_32)
        let s_5_33: Bits = Elem_read(state, tracer, s_5_29, s_5_31, s_5_32);
        // D s_5_34: read-var is_unsigned:u8
        let s_5_34: bool = fn_state.is_unsigned;
        // D s_5_35: call asl_Int(s_5_33, s_5_34)
        let s_5_35: i128 = asl_Int(state, tracer, s_5_33, s_5_34);
        // D s_5_36: sub s_5_17 s_5_35
        let s_5_36: i128 = ((s_5_17) - (s_5_35));
        // D s_5_37: read-var esizeshadow#7740:i64
        let s_5_37: i64 = fn_state.esizeshadow_7740;
        // D s_5_38: cast zx s_5_37 -> i
        let s_5_38: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_39: cast reint s_5_38 -> i64
        let s_5_39: i64 = (s_5_38 as i64);
        // D s_5_40: cast zx s_5_39 -> i
        let s_5_40: i128 = (i128::try_from(s_5_39).unwrap());
        // D s_5_41: read-var is_unsigned:u8
        let s_5_41: bool = fn_state.is_unsigned;
        // D s_5_42: call SatQ(s_5_36, s_5_40, s_5_41)
        let s_5_42: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_5_36,
            s_5_40,
            s_5_41,
        );
        // D s_5_43: write-var ga#359784 <= s_5_42
        fn_state.ga_359784 = s_5_42;
        // D s_5_44: read-var ga#359784.0:struct
        let s_5_44: Bits = fn_state.ga_359784._0;
        // D s_5_45: read-var ga#359784.1:struct
        let s_5_45: bool = fn_state.ga_359784._1;
        // D s_5_46: read-var r:i64
        let s_5_46: i64 = fn_state.r;
        // D s_5_47: cast zx s_5_46 -> i
        let s_5_47: i128 = (i128::try_from(s_5_46).unwrap());
        // D s_5_48: read-var d:i
        let s_5_48: i128 = fn_state.d;
        // D s_5_49: add s_5_48 s_5_47
        let s_5_49: i128 = (s_5_48 + s_5_47);
        // D s_5_50: read-var r:i64
        let s_5_50: i64 = fn_state.r;
        // D s_5_51: cast zx s_5_50 -> i
        let s_5_51: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_52: read-var d:i
        let s_5_52: i128 = fn_state.d;
        // D s_5_53: add s_5_52 s_5_51
        let s_5_53: i128 = (s_5_52 + s_5_51);
        // D s_5_54: call D_read(s_5_53)
        let s_5_54: u64 = D_read(state, tracer, s_5_53);
        // D s_5_55: read-var esizeshadow#7740:i64
        let s_5_55: i64 = fn_state.esizeshadow_7740;
        // D s_5_56: cast zx s_5_55 -> i
        let s_5_56: i128 = (i128::try_from(s_5_55).unwrap());
        // D s_5_57: cast reint s_5_56 -> i64
        let s_5_57: i64 = (s_5_56 as i64);
        // D s_5_58: cast zx s_5_54 -> bv
        let s_5_58: Bits = Bits::new(s_5_54 as u128, 64u16);
        // D s_5_59: read-var e:i64
        let s_5_59: i64 = fn_state.e;
        // D s_5_60: cast zx s_5_59 -> i
        let s_5_60: i128 = (i128::try_from(s_5_59).unwrap());
        // D s_5_61: cast zx s_5_57 -> i
        let s_5_61: i128 = (i128::try_from(s_5_57).unwrap());
        // D s_5_62: call Elem_set(s_5_58, s_5_60, s_5_61, s_5_44)
        let s_5_62: Bits = Elem_set(state, tracer, s_5_58, s_5_60, s_5_61, s_5_44);
        // D s_5_63: cast reint s_5_62 -> u64
        let s_5_63: u64 = (s_5_62.value() as u64);
        // D s_5_64: call D_set(s_5_49, s_5_63)
        let s_5_64: () = D_set(state, tracer, s_5_49, s_5_63);
        // N s_5_65: branch s_5_45 b8 b6
        if s_5_45 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call FPSCR_read__1(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_8_0);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // S s_8_3: call _update_FPSCR_Type_QC(s_8_1, s_8_2)
        let s_8_3: ProductType700c18a878c5601b = u_update_FPSCR_Type_QC(
            state,
            tracer,
            s_8_1,
            s_8_2,
        );
        // S s_8_4: call FPSCR_write(s_8_3)
        let s_8_4: () = FPSCR_write(state, tracer, s_8_3);
        // N s_8_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var r <= s_9_2
        fn_state.r = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
