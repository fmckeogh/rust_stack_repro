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
use X_set::*;
use asl_Int::*;
use DecodePredCount::*;
use X_read::*;
use CheckSVEEnabled::*;
use SatQ::*;
use common::*;
pub fn execute_UQDECW_R_RS_X<T: Tracer>(
    state: &mut State,
    tracer: &T,
    dn: i64,
    esize: i64,
    imm: i64,
    pat: u8,
    ssize: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_742310: ProductTypef506aa96a892fe52,
        result: Bits,
        ga_281532: u64,
        dn: i64,
        esize: i64,
        imm: i64,
        pat: u8,
        ssize: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        dn,
        esize,
        imm,
        pat,
        ssize,
        is_unsigned,
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
        // S s_0_1: call CheckSVEEnabled(s_0_0)
        let s_0_1: () = CheckSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var esize:i64
        let s_1_0: i64 = fn_state.esize;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var pat:u8
        let s_1_2: u8 = fn_state.pat;
        // D s_1_3: call DecodePredCount(s_1_2, s_1_1)
        let s_1_3: i128 = DecodePredCount(state, tracer, s_1_2, s_1_1);
        // D s_1_4: read-var ssize:i64
        let s_1_4: i64 = fn_state.ssize;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var dn:i64
        let s_1_7: i64 = fn_state.dn;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call X_read(s_1_8, s_1_6)
        let s_1_9: Bits = X_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: read-var is_unsigned:u8
        let s_1_10: bool = fn_state.is_unsigned;
        // D s_1_11: call asl_Int(s_1_9, s_1_10)
        let s_1_11: i128 = asl_Int(state, tracer, s_1_9, s_1_10);
        // D s_1_12: read-var imm:i64
        let s_1_12: i64 = fn_state.imm;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: mul s_1_3 s_1_13
        let s_1_14: i128 = ((s_1_3) * (s_1_13));
        // D s_1_15: sub s_1_11 s_1_14
        let s_1_15: i128 = ((s_1_11) - (s_1_14));
        // D s_1_16: read-var ssize:i64
        let s_1_16: i64 = fn_state.ssize;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: read-var is_unsigned:u8
        let s_1_20: bool = fn_state.is_unsigned;
        // D s_1_21: call SatQ(s_1_15, s_1_19, s_1_20)
        let s_1_21: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_1_15,
            s_1_19,
            s_1_20,
        );
        // D s_1_22: write-var gs#742310 <= s_1_21
        fn_state.gs_742310 = s_1_21;
        // D s_1_23: read-var gs#742310.0:struct
        let s_1_23: Bits = fn_state.gs_742310._0;
        // D s_1_24: cast reint s_1_23 -> u64
        let s_1_24: u64 = (s_1_23.value() as u64);
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 64u16);
        // D s_1_26: write-var result <= s_1_25
        fn_state.result = s_1_25;
        // D s_1_27: read-var is_unsigned:u8
        let s_1_27: bool = fn_state.is_unsigned;
        // N s_1_28: branch s_1_27 b4 b2
        if s_1_27 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i
        let s_2_0: i128 = 64;
        // D s_2_1: read-var result:bv
        let s_2_1: Bits = fn_state.result;
        // D s_2_2: bits-cast sx s_2_1 -> bv length s_2_0
        let s_2_2: Bits = s_2_1.sign_extend(s_2_0);
        // D s_2_3: cast reint s_2_2 -> u64
        let s_2_3: u64 = (s_2_2.value() as u64);
        // D s_2_4: write-var ga#281532 <= s_2_3
        fn_state.ga_281532 = s_2_3;
        // N s_2_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var dn:i64
        let s_3_0: i64 = fn_state.dn;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var ga#281532:u64
        let s_3_2: u64 = fn_state.ga_281532;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 64u16);
        // C s_3_4: const #64s : i64
        let s_3_4: i64 = 64;
        // D s_3_5: call X_set(s_3_1, s_3_4, s_3_3)
        let s_3_5: () = X_set(state, tracer, s_3_1, s_3_4, s_3_3);
        // N s_3_6: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i
        let s_4_0: i128 = 64;
        // D s_4_1: read-var result:bv
        let s_4_1: Bits = fn_state.result;
        // D s_4_2: bits-cast zx s_4_1 -> bv length s_4_0
        let s_4_2: Bits = s_4_1.zero_extend(s_4_0);
        // D s_4_3: cast reint s_4_2 -> u64
        let s_4_3: u64 = (s_4_2.value() as u64);
        // D s_4_4: write-var ga#281532 <= s_4_3
        fn_state.ga_281532 = s_4_3;
        // N s_4_5: jump b3
        return block_3(state, tracer, fn_state);
    }
}
