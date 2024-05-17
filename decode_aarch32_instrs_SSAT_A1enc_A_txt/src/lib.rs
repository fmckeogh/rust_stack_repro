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
use ConditionPassed::*;
use DecodeImmShift::*;
use execute_aarch32_instrs_SSAT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_SSAT_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    sat_imm: u8,
    Rd: u8,
    imm5: u8,
    sh: bool,
    Rn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_347403: ProductType396b95aa74979079,
        shift_nshadow_7266: i128,
        saturate_to: i64,
        shift_t: u32,
        n: i64,
        gs_301863: bool,
        d: i64,
        cond: u8,
        sat_imm: u8,
        Rd: u8,
        imm5: u8,
        sh: bool,
        Rn: u8,
    }
    let fn_state = FunctionState {
        cond,
        sat_imm,
        Rd,
        imm5,
        sh,
        Rn,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var Rd:u8
        let s_2_6: u8 = fn_state.Rd;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var d <= s_2_9
        fn_state.d = s_2_9;
        // D s_2_11: read-var Rn:u8
        let s_2_11: u8 = fn_state.Rn;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 4u16);
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (s_2_12.value() as i128);
        // D s_2_14: cast reint s_2_13 -> i64
        let s_2_14: i64 = (s_2_13 as i64);
        // D s_2_15: write-var n <= s_2_14
        fn_state.n = s_2_14;
        // D s_2_16: read-var sat_imm:u8
        let s_2_16: u8 = fn_state.sat_imm;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 5u16);
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (s_2_17.value() as i128);
        // D s_2_19: cast reint s_2_18 -> i64
        let s_2_19: i64 = (s_2_18 as i64);
        // C s_2_20: const #1s : i
        let s_2_20: i128 = 1;
        // D s_2_21: cast zx s_2_19 -> i
        let s_2_21: i128 = (i128::try_from(s_2_19).unwrap());
        // D s_2_22: add s_2_21 s_2_20
        let s_2_22: i128 = (s_2_21 + s_2_20);
        // D s_2_23: cast reint s_2_22 -> i64
        let s_2_23: i64 = (s_2_22 as i64);
        // D s_2_24: write-var saturate_to <= s_2_23
        fn_state.saturate_to = s_2_23;
        // D s_2_25: read-var sh:u8
        let s_2_25: bool = fn_state.sh;
        // D s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 1u16);
        // C s_2_27: const #0u : u8
        let s_2_27: bool = false;
        // C s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 1u16);
        // D s_2_29: cast reint s_2_26 -> u128
        let s_2_29: u128 = (s_2_26.value() as u128);
        // D s_2_30: size-of s_2_26
        let s_2_30: u16 = s_2_26.length();
        // C s_2_31: cast reint s_2_28 -> u128
        let s_2_31: u128 = (s_2_28.value() as u128);
        // D s_2_32: size-of s_2_28
        let s_2_32: u16 = s_2_28.length();
        // D s_2_33: lsl s_2_29 s_2_32
        let s_2_33: u128 = s_2_29 << s_2_32;
        // D s_2_34: or s_2_33 s_2_31
        let s_2_34: u128 = ((s_2_33) | (s_2_31));
        // D s_2_35: add s_2_30 s_2_32
        let s_2_35: u16 = (s_2_30 + s_2_32);
        // D s_2_36: create-bits s_2_34 s_2_35
        let s_2_36: Bits = Bits::new(s_2_34, s_2_35);
        // D s_2_37: cast reint s_2_36 -> u8
        let s_2_37: u8 = (s_2_36.value() as u8);
        // D s_2_38: read-var imm5:u8
        let s_2_38: u8 = fn_state.imm5;
        // D s_2_39: call DecodeImmShift(s_2_37, s_2_38)
        let s_2_39: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_2_37,
            s_2_38,
        );
        // D s_2_40: write-var ga#347403 <= s_2_39
        fn_state.ga_347403 = s_2_39;
        // D s_2_41: read-var ga#347403.0:struct
        let s_2_41: u32 = fn_state.ga_347403._0;
        // D s_2_42: read-var ga#347403.1:struct
        let s_2_42: i128 = fn_state.ga_347403._1;
        // D s_2_43: write-var shift_t <= s_2_41
        fn_state.shift_t = s_2_41;
        // D s_2_44: write-var shift_nshadow#7266 <= s_2_42
        fn_state.shift_nshadow_7266 = s_2_42;
        // C s_2_45: const #15s : i
        let s_2_45: i128 = 15;
        // D s_2_46: read-var d:i64
        let s_2_46: i64 = fn_state.d;
        // D s_2_47: cast zx s_2_46 -> i
        let s_2_47: i128 = (i128::try_from(s_2_46).unwrap());
        // D s_2_48: cmp-eq s_2_47 s_2_45
        let s_2_48: bool = ((s_2_47) == (s_2_45));
        // N s_2_49: branch s_2_48 b7 b3
        if s_2_48 {
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
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#301863 <= s_3_3
        fn_state.gs_301863 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#301863:u8
        let s_4_0: bool = fn_state.gs_301863;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var shift_t:u32
        let s_5_0: u32 = fn_state.shift_t;
        // D s_5_1: read-var shift_nshadow#7266:i
        let s_5_1: i128 = fn_state.shift_nshadow_7266;
        // D s_5_2: read-var saturate_to:i64
        let s_5_2: i64 = fn_state.saturate_to;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: read-var d:i64
        let s_5_5: i64 = fn_state.d;
        // D s_5_6: read-var n:i64
        let s_5_6: i64 = fn_state.n;
        // D s_5_7: call execute_aarch32_instrs_SSAT_Op_A_txt(s_5_5, s_5_6, s_5_4, s_5_1, s_5_0)
        let s_5_7: () = execute_aarch32_instrs_SSAT_Op_A_txt(
            state,
            tracer,
            s_5_5,
            s_5_6,
            s_5_4,
            s_5_1,
            s_5_0,
        );
        // N s_5_8: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#301863 <= s_7_0
        fn_state.gs_301863 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
