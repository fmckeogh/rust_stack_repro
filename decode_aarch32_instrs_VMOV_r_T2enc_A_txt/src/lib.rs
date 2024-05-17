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
use FPSCR_read__1::*;
use u_get_FPSCR_Type_length::*;
use ConditionPassed::*;
use u_get_FPSCR_Type_Stride::*;
use execute_aarch32_instrs_VMOV_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOV_r_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vd: u8,
    size: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        single_register: bool,
        regs: i64,
        d: i64,
        gs_313202: bool,
        D: bool,
        Vd: u8,
        size: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vd,
        size,
        M,
        Vm,
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call FPSCR_read__1(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_2_0);
        // S s_2_2: call _get_FPSCR_Type_length(s_2_1)
        let s_2_2: u8 = u_get_FPSCR_Type_length(state, tracer, s_2_1);
        // S s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 3u16);
        // C s_2_4: const #0u : u8
        let s_2_4: u8 = 0;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 3u16);
        // S s_2_6: cmp-ne s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) != (s_2_5));
        // N s_2_7: branch s_2_6 b10 b3
        if s_2_6 {
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
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call FPSCR_read__1(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_3_0);
        // S s_3_2: call _get_FPSCR_Type_Stride(s_3_1)
        let s_3_2: u8 = u_get_FPSCR_Type_Stride(state, tracer, s_3_1);
        // S s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // C s_3_4: const #0u : u8
        let s_3_4: u8 = 0;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // S s_3_6: cmp-ne s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) != (s_3_5));
        // D s_3_7: write-var gs#313202 <= s_3_6
        fn_state.gs_313202 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#313202:u8
        let s_4_0: bool = fn_state.gs_313202;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
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
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var single_register <= s_5_4
        fn_state.single_register = s_5_4;
        // C s_5_6: const #1s : i64
        let s_5_6: i64 = 1;
        // D s_5_7: write-var regs <= s_5_6
        fn_state.regs = s_5_6;
        // D s_5_8: read-var single_register:u8
        let s_5_8: bool = fn_state.single_register;
        // N s_5_9: branch s_5_8 b8 b6
        if s_5_8 {
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
        // D s_6_0: read-var D:u8
        let s_6_0: bool = fn_state.D;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // D s_6_2: read-var Vd:u8
        let s_6_2: u8 = fn_state.Vd;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 4u16);
        // D s_6_4: cast reint s_6_1 -> u128
        let s_6_4: u128 = (s_6_1.value() as u128);
        // D s_6_5: size-of s_6_1
        let s_6_5: u16 = s_6_1.length();
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: lsl s_6_4 s_6_7
        let s_6_8: u128 = s_6_4 << s_6_7;
        // D s_6_9: or s_6_8 s_6_6
        let s_6_9: u128 = ((s_6_8) | (s_6_6));
        // D s_6_10: add s_6_5 s_6_7
        let s_6_10: u16 = (s_6_5 + s_6_7);
        // D s_6_11: create-bits s_6_9 s_6_10
        let s_6_11: Bits = Bits::new(s_6_9, s_6_10);
        // D s_6_12: cast reint s_6_11 -> u8
        let s_6_12: u8 = (s_6_11.value() as u8);
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 5u16);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (s_6_13.value() as i128);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: write-var d <= s_6_15
        fn_state.d = s_6_15;
        // D s_6_17: read-var M:u8
        let s_6_17: bool = fn_state.M;
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // D s_6_19: read-var Vm:u8
        let s_6_19: u8 = fn_state.Vm;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 4u16);
        // D s_6_21: cast reint s_6_18 -> u128
        let s_6_21: u128 = (s_6_18.value() as u128);
        // D s_6_22: size-of s_6_18
        let s_6_22: u16 = s_6_18.length();
        // D s_6_23: cast reint s_6_20 -> u128
        let s_6_23: u128 = (s_6_20.value() as u128);
        // D s_6_24: size-of s_6_20
        let s_6_24: u16 = s_6_20.length();
        // D s_6_25: lsl s_6_21 s_6_24
        let s_6_25: u128 = s_6_21 << s_6_24;
        // D s_6_26: or s_6_25 s_6_23
        let s_6_26: u128 = ((s_6_25) | (s_6_23));
        // D s_6_27: add s_6_22 s_6_24
        let s_6_27: u16 = (s_6_22 + s_6_24);
        // D s_6_28: create-bits s_6_26 s_6_27
        let s_6_28: Bits = Bits::new(s_6_26, s_6_27);
        // D s_6_29: cast reint s_6_28 -> u8
        let s_6_29: u8 = (s_6_28.value() as u8);
        // D s_6_30: cast zx s_6_29 -> bv
        let s_6_30: Bits = Bits::new(s_6_29 as u128, 5u16);
        // D s_6_31: cast zx s_6_30 -> i
        let s_6_31: i128 = (s_6_30.value() as i128);
        // D s_6_32: cast reint s_6_31 -> i64
        let s_6_32: i64 = (s_6_31 as i64);
        // D s_6_33: write-var m <= s_6_32
        fn_state.m = s_6_32;
        // C s_6_34: const #1s : i64
        let s_6_34: i64 = 1;
        // D s_6_35: write-var regs <= s_6_34
        fn_state.regs = s_6_34;
        // N s_6_36: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var regs:i64
        let s_7_0: i64 = fn_state.regs;
        // D s_7_1: read-var m:i64
        let s_7_1: i64 = fn_state.m;
        // D s_7_2: read-var d:i64
        let s_7_2: i64 = fn_state.d;
        // C s_7_3: const #0u : u8
        let s_7_3: bool = false;
        // D s_7_4: read-var single_register:u8
        let s_7_4: bool = fn_state.single_register;
        // D s_7_5: call execute_aarch32_instrs_VMOV_r_Op_A_txt(s_7_3, s_7_2, s_7_1, s_7_0, s_7_4)
        let s_7_5: () = execute_aarch32_instrs_VMOV_r_Op_A_txt(
            state,
            tracer,
            s_7_3,
            s_7_2,
            s_7_1,
            s_7_0,
            s_7_4,
        );
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var Vd:u8
        let s_8_0: u8 = fn_state.Vd;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 4u16);
        // D s_8_2: read-var D:u8
        let s_8_2: bool = fn_state.D;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cast reint s_8_1 -> u128
        let s_8_4: u128 = (s_8_1.value() as u128);
        // D s_8_5: size-of s_8_1
        let s_8_5: u16 = s_8_1.length();
        // D s_8_6: cast reint s_8_3 -> u128
        let s_8_6: u128 = (s_8_3.value() as u128);
        // D s_8_7: size-of s_8_3
        let s_8_7: u16 = s_8_3.length();
        // D s_8_8: lsl s_8_4 s_8_7
        let s_8_8: u128 = s_8_4 << s_8_7;
        // D s_8_9: or s_8_8 s_8_6
        let s_8_9: u128 = ((s_8_8) | (s_8_6));
        // D s_8_10: add s_8_5 s_8_7
        let s_8_10: u16 = (s_8_5 + s_8_7);
        // D s_8_11: create-bits s_8_9 s_8_10
        let s_8_11: Bits = Bits::new(s_8_9, s_8_10);
        // D s_8_12: cast reint s_8_11 -> u8
        let s_8_12: u8 = (s_8_11.value() as u8);
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 5u16);
        // D s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (s_8_13.value() as i128);
        // D s_8_15: cast reint s_8_14 -> i64
        let s_8_15: i64 = (s_8_14 as i64);
        // D s_8_16: write-var d <= s_8_15
        fn_state.d = s_8_15;
        // D s_8_17: read-var Vm:u8
        let s_8_17: u8 = fn_state.Vm;
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 4u16);
        // D s_8_19: read-var M:u8
        let s_8_19: bool = fn_state.M;
        // D s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 1u16);
        // D s_8_21: cast reint s_8_18 -> u128
        let s_8_21: u128 = (s_8_18.value() as u128);
        // D s_8_22: size-of s_8_18
        let s_8_22: u16 = s_8_18.length();
        // D s_8_23: cast reint s_8_20 -> u128
        let s_8_23: u128 = (s_8_20.value() as u128);
        // D s_8_24: size-of s_8_20
        let s_8_24: u16 = s_8_20.length();
        // D s_8_25: lsl s_8_21 s_8_24
        let s_8_25: u128 = s_8_21 << s_8_24;
        // D s_8_26: or s_8_25 s_8_23
        let s_8_26: u128 = ((s_8_25) | (s_8_23));
        // D s_8_27: add s_8_22 s_8_24
        let s_8_27: u16 = (s_8_22 + s_8_24);
        // D s_8_28: create-bits s_8_26 s_8_27
        let s_8_28: Bits = Bits::new(s_8_26, s_8_27);
        // D s_8_29: cast reint s_8_28 -> u8
        let s_8_29: u8 = (s_8_28.value() as u8);
        // D s_8_30: cast zx s_8_29 -> bv
        let s_8_30: Bits = Bits::new(s_8_29 as u128, 5u16);
        // D s_8_31: cast zx s_8_30 -> i
        let s_8_31: i128 = (s_8_30.value() as i128);
        // D s_8_32: cast reint s_8_31 -> i64
        let s_8_32: i64 = (s_8_31 as i64);
        // D s_8_33: write-var m <= s_8_32
        fn_state.m = s_8_32;
        // N s_8_34: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#313202 <= s_10_0
        fn_state.gs_313202 = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
