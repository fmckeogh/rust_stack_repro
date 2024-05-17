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
use u_shr_int_general::*;
use asl_Int::*;
use D_set::*;
use Elem_read::*;
use FPSCR_read__1::*;
use FPSCR_write::*;
use D_read::*;
use Qin_read::*;
use common::*;
pub fn execute_aarch32_instrs_VQSHRN_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    dest_unsigned: bool,
    elements: i64,
    esize: i64,
    m: i64,
    shift_amount: i128,
    src_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_7733: i64,
        gs_317281: i64,
        d: i128,
        ga_359721: ProductTypef506aa96a892fe52,
        d__arg: i64,
        dest_unsigned: bool,
        elements: i64,
        esize: i64,
        m: i64,
        shift_amount: i128,
        src_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        dest_unsigned,
        elements,
        esize,
        m,
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
        // D s_0_3: write-var esizeshadow#7733 <= s_0_2
        fn_state.esizeshadow_7733 = s_0_2;
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
        // D s_1_2: read-var elements:i64
        let s_1_2: i64 = fn_state.elements;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#317281 <= s_1_5
        fn_state.gs_317281 = s_1_5;
        // D s_1_7: write-var e <= s_1_0
        fn_state.e = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#317281:i64
        let s_2_1: i64 = fn_state.gs_317281;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i64
        let s_3_0: i64 = 1;
        // D s_3_1: read-var m:i64
        let s_3_1: i64 = fn_state.m;
        // D s_3_2: lsr s_3_1 s_3_0
        let s_3_2: i64 = s_3_1 >> s_3_0;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: call Qin_read(s_3_3)
        let s_3_4: u128 = Qin_read(state, tracer, s_3_3);
        // C s_3_5: const #2s : i
        let s_3_5: i128 = 2;
        // D s_3_6: read-var esizeshadow#7733:i64
        let s_3_6: i64 = fn_state.esizeshadow_7733;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: mul s_3_5 s_3_7
        let s_3_8: i128 = ((s_3_5) * (s_3_7));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: cast zx s_3_4 -> bv
        let s_3_12: Bits = Bits::new(s_3_4 as u128, 128u16);
        // D s_3_13: read-var e:i64
        let s_3_13: i64 = fn_state.e;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast zx s_3_11 -> i
        let s_3_15: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_16: call Elem_read(s_3_12, s_3_14, s_3_15)
        let s_3_16: Bits = Elem_read(state, tracer, s_3_12, s_3_14, s_3_15);
        // D s_3_17: read-var src_unsigned:u8
        let s_3_17: bool = fn_state.src_unsigned;
        // D s_3_18: call asl_Int(s_3_16, s_3_17)
        let s_3_18: i128 = asl_Int(state, tracer, s_3_16, s_3_17);
        // D s_3_19: read-var shift_amount:i
        let s_3_19: i128 = fn_state.shift_amount;
        // D s_3_20: call _shr_int_general(s_3_18, s_3_19)
        let s_3_20: i128 = u_shr_int_general(state, tracer, s_3_18, s_3_19);
        // D s_3_21: read-var esizeshadow#7733:i64
        let s_3_21: i64 = fn_state.esizeshadow_7733;
        // D s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: read-var dest_unsigned:u8
        let s_3_25: bool = fn_state.dest_unsigned;
        // D s_3_26: call SatQ(s_3_20, s_3_24, s_3_25)
        let s_3_26: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_3_20,
            s_3_24,
            s_3_25,
        );
        // D s_3_27: write-var ga#359721 <= s_3_26
        fn_state.ga_359721 = s_3_26;
        // D s_3_28: read-var ga#359721.0:struct
        let s_3_28: Bits = fn_state.ga_359721._0;
        // D s_3_29: read-var ga#359721.1:struct
        let s_3_29: bool = fn_state.ga_359721._1;
        // D s_3_30: read-var d:i
        let s_3_30: i128 = fn_state.d;
        // D s_3_31: call D_read(s_3_30)
        let s_3_31: u64 = D_read(state, tracer, s_3_30);
        // D s_3_32: read-var esizeshadow#7733:i64
        let s_3_32: i64 = fn_state.esizeshadow_7733;
        // D s_3_33: cast zx s_3_32 -> i
        let s_3_33: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_34: cast reint s_3_33 -> i64
        let s_3_34: i64 = (s_3_33 as i64);
        // D s_3_35: cast zx s_3_31 -> bv
        let s_3_35: Bits = Bits::new(s_3_31 as u128, 64u16);
        // D s_3_36: read-var e:i64
        let s_3_36: i64 = fn_state.e;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cast zx s_3_34 -> i
        let s_3_38: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_39: call Elem_set(s_3_35, s_3_37, s_3_38, s_3_28)
        let s_3_39: Bits = Elem_set(state, tracer, s_3_35, s_3_37, s_3_38, s_3_28);
        // D s_3_40: cast reint s_3_39 -> u64
        let s_3_40: u64 = (s_3_39.value() as u64);
        // D s_3_41: read-var d:i
        let s_3_41: i128 = fn_state.d;
        // D s_3_42: call D_set(s_3_41, s_3_40)
        let s_3_42: () = D_set(state, tracer, s_3_41, s_3_40);
        // N s_3_43: branch s_3_29 b6 b4
        if s_3_29 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call FPSCR_read__1(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_6_0);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // S s_6_3: call _update_FPSCR_Type_QC(s_6_1, s_6_2)
        let s_6_3: ProductType700c18a878c5601b = u_update_FPSCR_Type_QC(
            state,
            tracer,
            s_6_1,
            s_6_2,
        );
        // S s_6_4: call FPSCR_write(s_6_3)
        let s_6_4: () = FPSCR_write(state, tracer, s_6_3);
        // N s_6_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
