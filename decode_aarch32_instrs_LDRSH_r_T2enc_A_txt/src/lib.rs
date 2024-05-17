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
use neq_int::*;
use execute_aarch32_instrs_LDRSH_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRSH_r_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    Rt: u8,
    imm2: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_344812: ProductType90c39552810120fd,
        t: i64,
        shift_nshadow_7193: i128,
        shift_t: u32,
        n: i64,
        Rn: u8,
        Rt: u8,
        imm2: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        Rn,
        Rt,
        imm2,
        Rm,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b8 b3
        if s_2_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // C s_3_2: const #15u : u8
        let s_3_2: u8 = 15;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b7 b4
        if s_3_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rt:u8
        let s_4_0: u8 = fn_state.Rt;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 4u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: write-var t <= s_4_3
        fn_state.t = s_4_3;
        // C s_4_5: const #15s : i
        let s_4_5: i128 = 15;
        // D s_4_6: read-var t:i64
        let s_4_6: i64 = fn_state.t;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: call neq_int(s_4_7, s_4_5)
        let s_4_8: bool = neq_int(state, tracer, s_4_7, s_4_5);
        // N s_4_9: assert s_4_8
        let s_4_9: () = assert!(s_4_8);
        // D s_4_10: read-var Rn:u8
        let s_4_10: u8 = fn_state.Rn;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 4u16);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (s_4_11.value() as i128);
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // D s_4_14: write-var n <= s_4_13
        fn_state.n = s_4_13;
        // D s_4_15: read-var Rm:u8
        let s_4_15: u8 = fn_state.Rm;
        // D s_4_16: cast zx s_4_15 -> bv
        let s_4_16: Bits = Bits::new(s_4_15 as u128, 4u16);
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (s_4_16.value() as i128);
        // D s_4_18: cast reint s_4_17 -> i64
        let s_4_18: i64 = (s_4_17 as i64);
        // D s_4_19: write-var m <= s_4_18
        fn_state.m = s_4_18;
        // D s_4_20: read-var imm2:u8
        let s_4_20: u8 = fn_state.imm2;
        // D s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 2u16);
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (s_4_21.value() as i128);
        // D s_4_23: cast reint s_4_22 -> i64
        let s_4_23: i64 = (s_4_22 as i64);
        // C s_4_24: const #0u : u32
        let s_4_24: u32 = 0;
        // D s_4_25: create-product struct = ["s_4_24", "s_4_23"]
        let s_4_25: ProductType90c39552810120fd = ProductType90c39552810120fd {
            _0: s_4_24,
            _1: s_4_23,
        };
        // D s_4_26: write-var ga#344812 <= s_4_25
        fn_state.ga_344812 = s_4_25;
        // D s_4_27: read-var ga#344812.0:struct
        let s_4_27: u32 = fn_state.ga_344812._0;
        // D s_4_28: read-var ga#344812.1:struct
        let s_4_28: i64 = fn_state.ga_344812._1;
        // D s_4_29: write-var shift_t <= s_4_27
        fn_state.shift_t = s_4_27;
        // D s_4_30: cast zx s_4_28 -> i
        let s_4_30: i128 = (i128::try_from(s_4_28).unwrap());
        // D s_4_31: write-var shift_nshadow#7193 <= s_4_30
        fn_state.shift_nshadow_7193 = s_4_30;
        // C s_4_32: const #15s : i
        let s_4_32: i128 = 15;
        // D s_4_33: read-var m:i64
        let s_4_33: i64 = fn_state.m;
        // D s_4_34: cast zx s_4_33 -> i
        let s_4_34: i128 = (i128::try_from(s_4_33).unwrap());
        // D s_4_35: cmp-eq s_4_34 s_4_32
        let s_4_35: bool = ((s_4_34) == (s_4_32));
        // N s_4_36: branch s_4_35 b6 b5
        if s_4_35 {
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
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // C s_5_1: const #1u : u8
        let s_5_1: bool = true;
        // D s_5_2: read-var m:i64
        let s_5_2: i64 = fn_state.m;
        // D s_5_3: read-var n:i64
        let s_5_3: i64 = fn_state.n;
        // D s_5_4: read-var shift_nshadow#7193:i
        let s_5_4: i128 = fn_state.shift_nshadow_7193;
        // D s_5_5: read-var shift_t:u32
        let s_5_5: u32 = fn_state.shift_t;
        // D s_5_6: read-var t:i64
        let s_5_6: i64 = fn_state.t;
        // C s_5_7: const #0u : u8
        let s_5_7: bool = false;
        // D s_5_8: call execute_aarch32_instrs_LDRSH_r_Op_A_txt(s_5_0, s_5_1, s_5_2, s_5_3, s_5_4, s_5_5, s_5_6, s_5_7)
        let s_5_8: () = execute_aarch32_instrs_LDRSH_r_Op_A_txt(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
            s_5_3,
            s_5_4,
            s_5_5,
            s_5_6,
            s_5_7,
        );
        // N s_5_9: return
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
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
}
