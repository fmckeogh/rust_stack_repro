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
use u_shl_int_general::*;
use D_set::*;
use asl_Int::*;
use FPSCR_read__1::*;
use Elem_read::*;
use FPSCR_write::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VQSHL_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    dest_unsigned: bool,
    elements: i64,
    esize: i64,
    m: i64,
    regs: i64,
    shift_amount: i128,
    src_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_317014: i64,
        r: i64,
        e: i64,
        ga_359564: ProductTypef506aa96a892fe52,
        esizeshadow_7725: i64,
        d: i128,
        gs_317020: i64,
        d__arg: i64,
        dest_unsigned: bool,
        elements: i64,
        esize: i64,
        m: i64,
        regs: i64,
        shift_amount: i128,
        src_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        dest_unsigned,
        elements,
        esize,
        m,
        regs,
        shift_amount,
        src_unsigned,
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
        // D s_0_3: write-var esizeshadow#7725 <= s_0_2
        fn_state.esizeshadow_7725 = s_0_2;
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
        // D s_1_6: write-var gs#317014 <= s_1_5
        fn_state.gs_317014 = s_1_5;
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
        // D s_2_1: read-var gs#317014:i64
        let s_2_1: i64 = fn_state.gs_317014;
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
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#317020 <= s_3_5
        fn_state.gs_317020 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#317020:i64
        let s_4_1: i64 = fn_state.gs_317020;
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
        // D s_5_0: read-var m:i64
        let s_5_0: i64 = fn_state.m;
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
        // D s_5_8: read-var esizeshadow#7725:i64
        let s_5_8: i64 = fn_state.esizeshadow_7725;
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
        // D s_5_16: read-var src_unsigned:u8
        let s_5_16: bool = fn_state.src_unsigned;
        // D s_5_17: call asl_Int(s_5_15, s_5_16)
        let s_5_17: i128 = asl_Int(state, tracer, s_5_15, s_5_16);
        // D s_5_18: read-var shift_amount:i
        let s_5_18: i128 = fn_state.shift_amount;
        // D s_5_19: call _shl_int_general(s_5_17, s_5_18)
        let s_5_19: i128 = u_shl_int_general(state, tracer, s_5_17, s_5_18);
        // D s_5_20: read-var esizeshadow#7725:i64
        let s_5_20: i64 = fn_state.esizeshadow_7725;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_24: read-var dest_unsigned:u8
        let s_5_24: bool = fn_state.dest_unsigned;
        // D s_5_25: call SatQ(s_5_19, s_5_23, s_5_24)
        let s_5_25: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_5_19,
            s_5_23,
            s_5_24,
        );
        // D s_5_26: write-var ga#359564 <= s_5_25
        fn_state.ga_359564 = s_5_25;
        // D s_5_27: read-var ga#359564.0:struct
        let s_5_27: Bits = fn_state.ga_359564._0;
        // D s_5_28: read-var ga#359564.1:struct
        let s_5_28: bool = fn_state.ga_359564._1;
        // D s_5_29: read-var r:i64
        let s_5_29: i64 = fn_state.r;
        // D s_5_30: cast zx s_5_29 -> i
        let s_5_30: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_31: read-var d:i
        let s_5_31: i128 = fn_state.d;
        // D s_5_32: add s_5_31 s_5_30
        let s_5_32: i128 = (s_5_31 + s_5_30);
        // D s_5_33: read-var r:i64
        let s_5_33: i64 = fn_state.r;
        // D s_5_34: cast zx s_5_33 -> i
        let s_5_34: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_35: read-var d:i
        let s_5_35: i128 = fn_state.d;
        // D s_5_36: add s_5_35 s_5_34
        let s_5_36: i128 = (s_5_35 + s_5_34);
        // D s_5_37: call D_read(s_5_36)
        let s_5_37: u64 = D_read(state, tracer, s_5_36);
        // D s_5_38: read-var esizeshadow#7725:i64
        let s_5_38: i64 = fn_state.esizeshadow_7725;
        // D s_5_39: cast zx s_5_38 -> i
        let s_5_39: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_40: cast reint s_5_39 -> i64
        let s_5_40: i64 = (s_5_39 as i64);
        // D s_5_41: cast zx s_5_37 -> bv
        let s_5_41: Bits = Bits::new(s_5_37 as u128, 64u16);
        // D s_5_42: read-var e:i64
        let s_5_42: i64 = fn_state.e;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: cast zx s_5_40 -> i
        let s_5_44: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_45: call Elem_set(s_5_41, s_5_43, s_5_44, s_5_27)
        let s_5_45: Bits = Elem_set(state, tracer, s_5_41, s_5_43, s_5_44, s_5_27);
        // D s_5_46: cast reint s_5_45 -> u64
        let s_5_46: u64 = (s_5_45.value() as u64);
        // D s_5_47: call D_set(s_5_32, s_5_46)
        let s_5_47: () = D_set(state, tracer, s_5_32, s_5_46);
        // N s_5_48: branch s_5_28 b8 b6
        if s_5_28 {
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
