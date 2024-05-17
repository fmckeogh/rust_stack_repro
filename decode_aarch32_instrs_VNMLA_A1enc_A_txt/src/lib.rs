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
use execute_aarch32_instrs_VNMLA_Op_A_txt::*;
use HaveFP16Ext::*;
use common::*;
pub fn decode_aarch32_instrs_VNMLA_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    D: bool,
    Vn: u8,
    Vd: u8,
    size: u8,
    N: bool,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        esize: i64,
        n: i64,
        gs_314505: bool,
        gs_314502: bool,
        d: i64,
        gs_314503: bool,
        vtype: u32,
        gs_314504: bool,
        cond: u8,
        D: bool,
        Vn: u8,
        Vd: u8,
        size: u8,
        N: bool,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        cond,
        D,
        Vn,
        Vd,
        size,
        N,
        op,
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
        // C s_2_6: const #() : ()
        let s_2_6: () = ();
        // S s_2_7: call FPSCR_read__1(s_2_6)
        let s_2_7: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_2_6);
        // S s_2_8: call _get_FPSCR_Type_length(s_2_7)
        let s_2_8: u8 = u_get_FPSCR_Type_length(state, tracer, s_2_7);
        // S s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 3u16);
        // C s_2_10: const #0u : u8
        let s_2_10: u8 = 0;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 3u16);
        // S s_2_12: cmp-ne s_2_9 s_2_11
        let s_2_12: bool = ((s_2_9) != (s_2_11));
        // N s_2_13: branch s_2_12 b30 b3
        if s_2_12 {
            return block_30(state, tracer, fn_state);
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
        // D s_3_7: write-var gs#314502 <= s_3_6
        fn_state.gs_314502 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#314502:u8
        let s_4_0: bool = fn_state.gs_314502;
        // N s_4_1: branch s_4_0 b29 b5
        if s_4_0 {
            return block_29(state, tracer, fn_state);
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
        // C s_5_2: const #0u : u8
        let s_5_2: u8 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b28 b6
        if s_5_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #1u : u8
        let s_6_2: u8 = 1;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b27 b7
        if s_6_4 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#314503 <= s_7_0
        fn_state.gs_314503 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#314503:u8
        let s_8_0: bool = fn_state.gs_314503;
        // D s_8_1: write-var gs#314504 <= s_8_0
        fn_state.gs_314504 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#314504:u8
        let s_9_0: bool = fn_state.gs_314504;
        // N s_9_1: branch s_9_0 b26 b10
        if s_9_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:u8
        let s_10_0: u8 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #1u : u8
        let s_10_2: u8 = 1;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b25 b11
        if s_10_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#314505 <= s_11_0
        fn_state.gs_314505 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#314505:u8
        let s_12_0: bool = fn_state.gs_314505;
        // N s_12_1: branch s_12_0 b24 b13
        if s_12_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var op:u8
        let s_13_0: bool = fn_state.op;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // N s_13_5: branch s_13_4 b23 b14
        if s_13_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // D s_14_1: write-var vtype <= s_14_0
        fn_state.vtype = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16s : i64
        let s_15_0: i64 = 16;
        // D s_15_1: write-var esize <= s_15_0
        fn_state.esize = s_15_0;
        // D s_15_2: read-var size:u8
        let s_15_2: u8 = fn_state.size;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // C s_15_4: const #1u : u8
        let s_15_4: u8 = 1;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: not s_15_6
        let s_15_7: bool = !s_15_6;
        // N s_15_8: branch s_15_7 b18 b16
        if s_15_7 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16s : i64
        let s_16_0: i64 = 16;
        // D s_16_1: write-var esize <= s_16_0
        fn_state.esize = s_16_0;
        // D s_16_2: read-var Vd:u8
        let s_16_2: u8 = fn_state.Vd;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 4u16);
        // D s_16_4: read-var D:u8
        let s_16_4: bool = fn_state.D;
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cast reint s_16_3 -> u128
        let s_16_6: u128 = (s_16_3.value() as u128);
        // D s_16_7: size-of s_16_3
        let s_16_7: u16 = s_16_3.length();
        // D s_16_8: cast reint s_16_5 -> u128
        let s_16_8: u128 = (s_16_5.value() as u128);
        // D s_16_9: size-of s_16_5
        let s_16_9: u16 = s_16_5.length();
        // D s_16_10: lsl s_16_6 s_16_9
        let s_16_10: u128 = s_16_6 << s_16_9;
        // D s_16_11: or s_16_10 s_16_8
        let s_16_11: u128 = ((s_16_10) | (s_16_8));
        // D s_16_12: add s_16_7 s_16_9
        let s_16_12: u16 = (s_16_7 + s_16_9);
        // D s_16_13: create-bits s_16_11 s_16_12
        let s_16_13: Bits = Bits::new(s_16_11, s_16_12);
        // D s_16_14: cast reint s_16_13 -> u8
        let s_16_14: u8 = (s_16_13.value() as u8);
        // D s_16_15: cast zx s_16_14 -> bv
        let s_16_15: Bits = Bits::new(s_16_14 as u128, 5u16);
        // D s_16_16: cast zx s_16_15 -> i
        let s_16_16: i128 = (s_16_15.value() as i128);
        // D s_16_17: cast reint s_16_16 -> i64
        let s_16_17: i64 = (s_16_16 as i64);
        // D s_16_18: write-var d <= s_16_17
        fn_state.d = s_16_17;
        // D s_16_19: read-var Vn:u8
        let s_16_19: u8 = fn_state.Vn;
        // D s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 4u16);
        // D s_16_21: read-var N:u8
        let s_16_21: bool = fn_state.N;
        // D s_16_22: cast zx s_16_21 -> bv
        let s_16_22: Bits = Bits::new(s_16_21 as u128, 1u16);
        // D s_16_23: cast reint s_16_20 -> u128
        let s_16_23: u128 = (s_16_20.value() as u128);
        // D s_16_24: size-of s_16_20
        let s_16_24: u16 = s_16_20.length();
        // D s_16_25: cast reint s_16_22 -> u128
        let s_16_25: u128 = (s_16_22.value() as u128);
        // D s_16_26: size-of s_16_22
        let s_16_26: u16 = s_16_22.length();
        // D s_16_27: lsl s_16_23 s_16_26
        let s_16_27: u128 = s_16_23 << s_16_26;
        // D s_16_28: or s_16_27 s_16_25
        let s_16_28: u128 = ((s_16_27) | (s_16_25));
        // D s_16_29: add s_16_24 s_16_26
        let s_16_29: u16 = (s_16_24 + s_16_26);
        // D s_16_30: create-bits s_16_28 s_16_29
        let s_16_30: Bits = Bits::new(s_16_28, s_16_29);
        // D s_16_31: cast reint s_16_30 -> u8
        let s_16_31: u8 = (s_16_30.value() as u8);
        // D s_16_32: cast zx s_16_31 -> bv
        let s_16_32: Bits = Bits::new(s_16_31 as u128, 5u16);
        // D s_16_33: cast zx s_16_32 -> i
        let s_16_33: i128 = (s_16_32.value() as i128);
        // D s_16_34: cast reint s_16_33 -> i64
        let s_16_34: i64 = (s_16_33 as i64);
        // D s_16_35: write-var n <= s_16_34
        fn_state.n = s_16_34;
        // D s_16_36: read-var Vm:u8
        let s_16_36: u8 = fn_state.Vm;
        // D s_16_37: cast zx s_16_36 -> bv
        let s_16_37: Bits = Bits::new(s_16_36 as u128, 4u16);
        // D s_16_38: read-var M:u8
        let s_16_38: bool = fn_state.M;
        // D s_16_39: cast zx s_16_38 -> bv
        let s_16_39: Bits = Bits::new(s_16_38 as u128, 1u16);
        // D s_16_40: cast reint s_16_37 -> u128
        let s_16_40: u128 = (s_16_37.value() as u128);
        // D s_16_41: size-of s_16_37
        let s_16_41: u16 = s_16_37.length();
        // D s_16_42: cast reint s_16_39 -> u128
        let s_16_42: u128 = (s_16_39.value() as u128);
        // D s_16_43: size-of s_16_39
        let s_16_43: u16 = s_16_39.length();
        // D s_16_44: lsl s_16_40 s_16_43
        let s_16_44: u128 = s_16_40 << s_16_43;
        // D s_16_45: or s_16_44 s_16_42
        let s_16_45: u128 = ((s_16_44) | (s_16_42));
        // D s_16_46: add s_16_41 s_16_43
        let s_16_46: u16 = (s_16_41 + s_16_43);
        // D s_16_47: create-bits s_16_45 s_16_46
        let s_16_47: Bits = Bits::new(s_16_45, s_16_46);
        // D s_16_48: cast reint s_16_47 -> u8
        let s_16_48: u8 = (s_16_47.value() as u8);
        // D s_16_49: cast zx s_16_48 -> bv
        let s_16_49: Bits = Bits::new(s_16_48 as u128, 5u16);
        // D s_16_50: cast zx s_16_49 -> i
        let s_16_50: i128 = (s_16_49.value() as i128);
        // D s_16_51: cast reint s_16_50 -> i64
        let s_16_51: i64 = (s_16_50 as i64);
        // D s_16_52: write-var m <= s_16_51
        fn_state.m = s_16_51;
        // N s_16_53: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var n:i64
        let s_17_0: i64 = fn_state.n;
        // D s_17_1: read-var m:i64
        let s_17_1: i64 = fn_state.m;
        // D s_17_2: read-var esize:i64
        let s_17_2: i64 = fn_state.esize;
        // D s_17_3: read-var d:i64
        let s_17_3: i64 = fn_state.d;
        // D s_17_4: read-var vtype:u32
        let s_17_4: u32 = fn_state.vtype;
        // D s_17_5: call execute_aarch32_instrs_VNMLA_Op_A_txt(s_17_3, s_17_2, s_17_1, s_17_0, s_17_4)
        let s_17_5: () = execute_aarch32_instrs_VNMLA_Op_A_txt(
            state,
            tracer,
            s_17_3,
            s_17_2,
            s_17_1,
            s_17_0,
            s_17_4,
        );
        // N s_17_6: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var size:u8
        let s_18_0: u8 = fn_state.size;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #32s : i64
        let s_19_0: i64 = 32;
        // D s_19_1: write-var esize <= s_19_0
        fn_state.esize = s_19_0;
        // D s_19_2: read-var Vd:u8
        let s_19_2: u8 = fn_state.Vd;
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: read-var D:u8
        let s_19_4: bool = fn_state.D;
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cast reint s_19_3 -> u128
        let s_19_6: u128 = (s_19_3.value() as u128);
        // D s_19_7: size-of s_19_3
        let s_19_7: u16 = s_19_3.length();
        // D s_19_8: cast reint s_19_5 -> u128
        let s_19_8: u128 = (s_19_5.value() as u128);
        // D s_19_9: size-of s_19_5
        let s_19_9: u16 = s_19_5.length();
        // D s_19_10: lsl s_19_6 s_19_9
        let s_19_10: u128 = s_19_6 << s_19_9;
        // D s_19_11: or s_19_10 s_19_8
        let s_19_11: u128 = ((s_19_10) | (s_19_8));
        // D s_19_12: add s_19_7 s_19_9
        let s_19_12: u16 = (s_19_7 + s_19_9);
        // D s_19_13: create-bits s_19_11 s_19_12
        let s_19_13: Bits = Bits::new(s_19_11, s_19_12);
        // D s_19_14: cast reint s_19_13 -> u8
        let s_19_14: u8 = (s_19_13.value() as u8);
        // D s_19_15: cast zx s_19_14 -> bv
        let s_19_15: Bits = Bits::new(s_19_14 as u128, 5u16);
        // D s_19_16: cast zx s_19_15 -> i
        let s_19_16: i128 = (s_19_15.value() as i128);
        // D s_19_17: cast reint s_19_16 -> i64
        let s_19_17: i64 = (s_19_16 as i64);
        // D s_19_18: write-var d <= s_19_17
        fn_state.d = s_19_17;
        // D s_19_19: read-var Vn:u8
        let s_19_19: u8 = fn_state.Vn;
        // D s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 4u16);
        // D s_19_21: read-var N:u8
        let s_19_21: bool = fn_state.N;
        // D s_19_22: cast zx s_19_21 -> bv
        let s_19_22: Bits = Bits::new(s_19_21 as u128, 1u16);
        // D s_19_23: cast reint s_19_20 -> u128
        let s_19_23: u128 = (s_19_20.value() as u128);
        // D s_19_24: size-of s_19_20
        let s_19_24: u16 = s_19_20.length();
        // D s_19_25: cast reint s_19_22 -> u128
        let s_19_25: u128 = (s_19_22.value() as u128);
        // D s_19_26: size-of s_19_22
        let s_19_26: u16 = s_19_22.length();
        // D s_19_27: lsl s_19_23 s_19_26
        let s_19_27: u128 = s_19_23 << s_19_26;
        // D s_19_28: or s_19_27 s_19_25
        let s_19_28: u128 = ((s_19_27) | (s_19_25));
        // D s_19_29: add s_19_24 s_19_26
        let s_19_29: u16 = (s_19_24 + s_19_26);
        // D s_19_30: create-bits s_19_28 s_19_29
        let s_19_30: Bits = Bits::new(s_19_28, s_19_29);
        // D s_19_31: cast reint s_19_30 -> u8
        let s_19_31: u8 = (s_19_30.value() as u8);
        // D s_19_32: cast zx s_19_31 -> bv
        let s_19_32: Bits = Bits::new(s_19_31 as u128, 5u16);
        // D s_19_33: cast zx s_19_32 -> i
        let s_19_33: i128 = (s_19_32.value() as i128);
        // D s_19_34: cast reint s_19_33 -> i64
        let s_19_34: i64 = (s_19_33 as i64);
        // D s_19_35: write-var n <= s_19_34
        fn_state.n = s_19_34;
        // D s_19_36: read-var Vm:u8
        let s_19_36: u8 = fn_state.Vm;
        // D s_19_37: cast zx s_19_36 -> bv
        let s_19_37: Bits = Bits::new(s_19_36 as u128, 4u16);
        // D s_19_38: read-var M:u8
        let s_19_38: bool = fn_state.M;
        // D s_19_39: cast zx s_19_38 -> bv
        let s_19_39: Bits = Bits::new(s_19_38 as u128, 1u16);
        // D s_19_40: cast reint s_19_37 -> u128
        let s_19_40: u128 = (s_19_37.value() as u128);
        // D s_19_41: size-of s_19_37
        let s_19_41: u16 = s_19_37.length();
        // D s_19_42: cast reint s_19_39 -> u128
        let s_19_42: u128 = (s_19_39.value() as u128);
        // D s_19_43: size-of s_19_39
        let s_19_43: u16 = s_19_39.length();
        // D s_19_44: lsl s_19_40 s_19_43
        let s_19_44: u128 = s_19_40 << s_19_43;
        // D s_19_45: or s_19_44 s_19_42
        let s_19_45: u128 = ((s_19_44) | (s_19_42));
        // D s_19_46: add s_19_41 s_19_43
        let s_19_46: u16 = (s_19_41 + s_19_43);
        // D s_19_47: create-bits s_19_45 s_19_46
        let s_19_47: Bits = Bits::new(s_19_45, s_19_46);
        // D s_19_48: cast reint s_19_47 -> u8
        let s_19_48: u8 = (s_19_47.value() as u8);
        // D s_19_49: cast zx s_19_48 -> bv
        let s_19_49: Bits = Bits::new(s_19_48 as u128, 5u16);
        // D s_19_50: cast zx s_19_49 -> i
        let s_19_50: i128 = (s_19_49.value() as i128);
        // D s_19_51: cast reint s_19_50 -> i64
        let s_19_51: i64 = (s_19_50 as i64);
        // D s_19_52: write-var m <= s_19_51
        fn_state.m = s_19_51;
        // N s_19_53: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var size:u8
        let s_20_0: u8 = fn_state.size;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #3u : u8
        let s_20_2: u8 = 3;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 2u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // D s_21_1: write-var esize <= s_21_0
        fn_state.esize = s_21_0;
        // D s_21_2: read-var D:u8
        let s_21_2: bool = fn_state.D;
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: read-var Vd:u8
        let s_21_4: u8 = fn_state.Vd;
        // D s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 4u16);
        // D s_21_6: cast reint s_21_3 -> u128
        let s_21_6: u128 = (s_21_3.value() as u128);
        // D s_21_7: size-of s_21_3
        let s_21_7: u16 = s_21_3.length();
        // D s_21_8: cast reint s_21_5 -> u128
        let s_21_8: u128 = (s_21_5.value() as u128);
        // D s_21_9: size-of s_21_5
        let s_21_9: u16 = s_21_5.length();
        // D s_21_10: lsl s_21_6 s_21_9
        let s_21_10: u128 = s_21_6 << s_21_9;
        // D s_21_11: or s_21_10 s_21_8
        let s_21_11: u128 = ((s_21_10) | (s_21_8));
        // D s_21_12: add s_21_7 s_21_9
        let s_21_12: u16 = (s_21_7 + s_21_9);
        // D s_21_13: create-bits s_21_11 s_21_12
        let s_21_13: Bits = Bits::new(s_21_11, s_21_12);
        // D s_21_14: cast reint s_21_13 -> u8
        let s_21_14: u8 = (s_21_13.value() as u8);
        // D s_21_15: cast zx s_21_14 -> bv
        let s_21_15: Bits = Bits::new(s_21_14 as u128, 5u16);
        // D s_21_16: cast zx s_21_15 -> i
        let s_21_16: i128 = (s_21_15.value() as i128);
        // D s_21_17: cast reint s_21_16 -> i64
        let s_21_17: i64 = (s_21_16 as i64);
        // D s_21_18: write-var d <= s_21_17
        fn_state.d = s_21_17;
        // D s_21_19: read-var N:u8
        let s_21_19: bool = fn_state.N;
        // D s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 1u16);
        // D s_21_21: read-var Vn:u8
        let s_21_21: u8 = fn_state.Vn;
        // D s_21_22: cast zx s_21_21 -> bv
        let s_21_22: Bits = Bits::new(s_21_21 as u128, 4u16);
        // D s_21_23: cast reint s_21_20 -> u128
        let s_21_23: u128 = (s_21_20.value() as u128);
        // D s_21_24: size-of s_21_20
        let s_21_24: u16 = s_21_20.length();
        // D s_21_25: cast reint s_21_22 -> u128
        let s_21_25: u128 = (s_21_22.value() as u128);
        // D s_21_26: size-of s_21_22
        let s_21_26: u16 = s_21_22.length();
        // D s_21_27: lsl s_21_23 s_21_26
        let s_21_27: u128 = s_21_23 << s_21_26;
        // D s_21_28: or s_21_27 s_21_25
        let s_21_28: u128 = ((s_21_27) | (s_21_25));
        // D s_21_29: add s_21_24 s_21_26
        let s_21_29: u16 = (s_21_24 + s_21_26);
        // D s_21_30: create-bits s_21_28 s_21_29
        let s_21_30: Bits = Bits::new(s_21_28, s_21_29);
        // D s_21_31: cast reint s_21_30 -> u8
        let s_21_31: u8 = (s_21_30.value() as u8);
        // D s_21_32: cast zx s_21_31 -> bv
        let s_21_32: Bits = Bits::new(s_21_31 as u128, 5u16);
        // D s_21_33: cast zx s_21_32 -> i
        let s_21_33: i128 = (s_21_32.value() as i128);
        // D s_21_34: cast reint s_21_33 -> i64
        let s_21_34: i64 = (s_21_33 as i64);
        // D s_21_35: write-var n <= s_21_34
        fn_state.n = s_21_34;
        // D s_21_36: read-var M:u8
        let s_21_36: bool = fn_state.M;
        // D s_21_37: cast zx s_21_36 -> bv
        let s_21_37: Bits = Bits::new(s_21_36 as u128, 1u16);
        // D s_21_38: read-var Vm:u8
        let s_21_38: u8 = fn_state.Vm;
        // D s_21_39: cast zx s_21_38 -> bv
        let s_21_39: Bits = Bits::new(s_21_38 as u128, 4u16);
        // D s_21_40: cast reint s_21_37 -> u128
        let s_21_40: u128 = (s_21_37.value() as u128);
        // D s_21_41: size-of s_21_37
        let s_21_41: u16 = s_21_37.length();
        // D s_21_42: cast reint s_21_39 -> u128
        let s_21_42: u128 = (s_21_39.value() as u128);
        // D s_21_43: size-of s_21_39
        let s_21_43: u16 = s_21_39.length();
        // D s_21_44: lsl s_21_40 s_21_43
        let s_21_44: u128 = s_21_40 << s_21_43;
        // D s_21_45: or s_21_44 s_21_42
        let s_21_45: u128 = ((s_21_44) | (s_21_42));
        // D s_21_46: add s_21_41 s_21_43
        let s_21_46: u16 = (s_21_41 + s_21_43);
        // D s_21_47: create-bits s_21_45 s_21_46
        let s_21_47: Bits = Bits::new(s_21_45, s_21_46);
        // D s_21_48: cast reint s_21_47 -> u8
        let s_21_48: u8 = (s_21_47.value() as u8);
        // D s_21_49: cast zx s_21_48 -> bv
        let s_21_49: Bits = Bits::new(s_21_48 as u128, 5u16);
        // D s_21_50: cast zx s_21_49 -> i
        let s_21_50: i128 = (s_21_49.value() as i128);
        // D s_21_51: cast reint s_21_50 -> i64
        let s_21_51: i64 = (s_21_50 as i64);
        // D s_21_52: write-var m <= s_21_51
        fn_state.m = s_21_51;
        // N s_21_53: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u32
        let s_23_0: u32 = 0;
        // D s_23_1: write-var vtype <= s_23_0
        fn_state.vtype = s_23_0;
        // N s_23_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var cond:u8
        let s_25_0: u8 = fn_state.cond;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 4u16);
        // C s_25_2: const #14u : u8
        let s_25_2: u8 = 14;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 4u16);
        // D s_25_4: cmp-ne s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) != (s_25_3));
        // D s_25_5: write-var gs#314505 <= s_25_4
        fn_state.gs_314505 = s_25_4;
        // N s_25_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call HaveFP16Ext(s_27_0)
        let s_27_1: bool = HaveFP16Ext(state, tracer, s_27_0);
        // S s_27_2: not s_27_1
        let s_27_2: bool = !s_27_1;
        // D s_27_3: write-var gs#314503 <= s_27_2
        fn_state.gs_314503 = s_27_2;
        // N s_27_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#314504 <= s_28_0
        fn_state.gs_314504 = s_28_0;
        // N s_28_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#314502 <= s_30_0
        fn_state.gs_314502 = s_30_0;
        // N s_30_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
