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
use u_get_HFGWTR_EL2_Type_PAR_EL1::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use PAR_EL1_read::*;
use PAR_EL1_write::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use Mk_PAR_EL1_Type::*;
use EL2Enabled::*;
use common::*;
pub fn PAR_EL1_SysRegWrite_0aa8c34fdfcc39e2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_87102: bool,
        u__HFGWTR_EL2_PAR_EL1: bool,
        gs_87103: bool,
        ga_122847: ProductType782ac6922b48c20d,
        gs_87101: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_87104: bool,
        ga_122833: ProductType782ac6922b48c20d,
        u__PSTATE_EL: u8,
        ga_122840: ProductType782ac6922b48c20d,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var __PSTATE_EL <= s_0_1
        fn_state.u__PSTATE_EL = s_0_1;
        // C s_0_3: const #90704u : u32
        let s_0_3: u32 = 90704;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_EL3_Type_FGTEn(s_0_4)
        let s_0_5: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_4);
        // D s_0_6: write-var __SCR_EL3_FGTEn <= s_0_5
        fn_state.u__SCR_EL3_FGTEn = s_0_5;
        // C s_0_7: const #100992u : u32
        let s_0_7: u32 = 100992;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HFGWTR_EL2_Type_PAR_EL1(s_0_8)
        let s_0_9: bool = u_get_HFGWTR_EL2_Type_PAR_EL1(state, tracer, s_0_8);
        // D s_0_10: write-var __HFGWTR_EL2_PAR_EL1 <= s_0_9
        fn_state.u__HFGWTR_EL2_PAR_EL1 = s_0_9;
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b22 b1
        if s_0_16 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var __PSTATE_EL:u8
        let s_1_0: u8 = fn_state.u__PSTATE_EL;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #440u : u32
        let s_1_2: u32 = 440;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var __PSTATE_EL:u8
        let s_2_0: u8 = fn_state.u__PSTATE_EL;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #432u : u32
        let s_2_2: u32 = 432;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b6 b3
        if s_2_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call PAR_EL1_read(s_5_0)
        let s_5_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_5_0);
        // D s_5_2: write-var ga#122847 <= s_5_1
        fn_state.ga_122847 = s_5_1;
        // D s_5_3: read-var ga#122847.0:struct
        let s_5_3: u128 = fn_state.ga_122847._0;
        // C s_5_4: const #64s : i64
        let s_5_4: i64 = 64;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call X_read(s_5_5, s_5_4)
        let s_5_6: Bits = X_read(state, tracer, s_5_5, s_5_4);
        // D s_5_7: cast reint s_5_6 -> u64
        let s_5_7: u64 = (s_5_6.value() as u64);
        // C s_5_8: const #0s : i
        let s_5_8: i128 = 0;
        // D s_5_9: cast zx s_5_3 -> bv
        let s_5_9: Bits = Bits::new(s_5_3 as u128, 128u16);
        // D s_5_10: cast zx s_5_7 -> bv
        let s_5_10: Bits = Bits::new(s_5_7 as u128, 64u16);
        // C s_5_11: const #63s : i
        let s_5_11: i128 = 63;
        // C s_5_12: const #1u : u64
        let s_5_12: u64 = 1;
        // C s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 64u16);
        // C s_5_14: lsl s_5_13 s_5_11
        let s_5_14: Bits = s_5_13 << s_5_11;
        // C s_5_15: sub s_5_14 s_5_13
        let s_5_15: Bits = ((s_5_14) - (s_5_13));
        // D s_5_16: and s_5_10 s_5_15
        let s_5_16: Bits = ((s_5_10) & (s_5_15));
        // D s_5_17: lsl s_5_16 s_5_8
        let s_5_17: Bits = s_5_16 << s_5_8;
        // C s_5_18: lsl s_5_15 s_5_8
        let s_5_18: Bits = s_5_15 << s_5_8;
        // C s_5_19: cmpl s_5_18
        let s_5_19: Bits = !s_5_18;
        // D s_5_20: and s_5_9 s_5_19
        let s_5_20: Bits = ((s_5_9) & (s_5_19));
        // D s_5_21: or s_5_20 s_5_17
        let s_5_21: Bits = ((s_5_20) | (s_5_17));
        // D s_5_22: cast reint s_5_21 -> u128
        let s_5_22: u128 = (s_5_21.value() as u128);
        // D s_5_23: call Mk_PAR_EL1_Type(s_5_22)
        let s_5_23: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(state, tracer, s_5_22);
        // D s_5_24: call PAR_EL1_write(s_5_23)
        let s_5_24: () = PAR_EL1_write(state, tracer, s_5_23);
        // N s_5_25: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call PAR_EL1_read(s_6_0)
        let s_6_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_6_0);
        // D s_6_2: write-var ga#122840 <= s_6_1
        fn_state.ga_122840 = s_6_1;
        // D s_6_3: read-var ga#122840.0:struct
        let s_6_3: u128 = fn_state.ga_122840._0;
        // C s_6_4: const #64s : i64
        let s_6_4: i64 = 64;
        // D s_6_5: read-var t:i
        let s_6_5: i128 = fn_state.t;
        // D s_6_6: call X_read(s_6_5, s_6_4)
        let s_6_6: Bits = X_read(state, tracer, s_6_5, s_6_4);
        // D s_6_7: cast reint s_6_6 -> u64
        let s_6_7: u64 = (s_6_6.value() as u64);
        // C s_6_8: const #0s : i
        let s_6_8: i128 = 0;
        // D s_6_9: cast zx s_6_3 -> bv
        let s_6_9: Bits = Bits::new(s_6_3 as u128, 128u16);
        // D s_6_10: cast zx s_6_7 -> bv
        let s_6_10: Bits = Bits::new(s_6_7 as u128, 64u16);
        // C s_6_11: const #63s : i
        let s_6_11: i128 = 63;
        // C s_6_12: const #1u : u64
        let s_6_12: u64 = 1;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 64u16);
        // C s_6_14: lsl s_6_13 s_6_11
        let s_6_14: Bits = s_6_13 << s_6_11;
        // C s_6_15: sub s_6_14 s_6_13
        let s_6_15: Bits = ((s_6_14) - (s_6_13));
        // D s_6_16: and s_6_10 s_6_15
        let s_6_16: Bits = ((s_6_10) & (s_6_15));
        // D s_6_17: lsl s_6_16 s_6_8
        let s_6_17: Bits = s_6_16 << s_6_8;
        // C s_6_18: lsl s_6_15 s_6_8
        let s_6_18: Bits = s_6_15 << s_6_8;
        // C s_6_19: cmpl s_6_18
        let s_6_19: Bits = !s_6_18;
        // D s_6_20: and s_6_9 s_6_19
        let s_6_20: Bits = ((s_6_9) & (s_6_19));
        // D s_6_21: or s_6_20 s_6_17
        let s_6_21: Bits = ((s_6_20) | (s_6_17));
        // D s_6_22: cast reint s_6_21 -> u128
        let s_6_22: u128 = (s_6_21.value() as u128);
        // D s_6_23: call Mk_PAR_EL1_Type(s_6_22)
        let s_6_23: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(state, tracer, s_6_22);
        // D s_6_24: call PAR_EL1_write(s_6_23)
        let s_6_24: () = PAR_EL1_write(state, tracer, s_6_23);
        // N s_6_25: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call EL2Enabled(s_7_0)
        let s_7_1: bool = EL2Enabled(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b21 b8
        if s_7_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#87101 <= s_8_0
        fn_state.gs_87101 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#87101:u8
        let s_9_0: bool = fn_state.gs_87101;
        // N s_9_1: branch s_9_0 b17 b10
        if s_9_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#87103 <= s_10_0
        fn_state.gs_87103 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#87103:u8
        let s_11_0: bool = fn_state.gs_87103;
        // N s_11_1: branch s_11_0 b16 b12
        if s_11_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#87104 <= s_12_0
        fn_state.gs_87104 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#87104:u8
        let s_13_0: bool = fn_state.gs_87104;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call PAR_EL1_read(s_14_0)
        let s_14_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_14_0);
        // D s_14_2: write-var ga#122833 <= s_14_1
        fn_state.ga_122833 = s_14_1;
        // D s_14_3: read-var ga#122833.0:struct
        let s_14_3: u128 = fn_state.ga_122833._0;
        // C s_14_4: const #64s : i64
        let s_14_4: i64 = 64;
        // D s_14_5: read-var t:i
        let s_14_5: i128 = fn_state.t;
        // D s_14_6: call X_read(s_14_5, s_14_4)
        let s_14_6: Bits = X_read(state, tracer, s_14_5, s_14_4);
        // D s_14_7: cast reint s_14_6 -> u64
        let s_14_7: u64 = (s_14_6.value() as u64);
        // C s_14_8: const #0s : i
        let s_14_8: i128 = 0;
        // D s_14_9: cast zx s_14_3 -> bv
        let s_14_9: Bits = Bits::new(s_14_3 as u128, 128u16);
        // D s_14_10: cast zx s_14_7 -> bv
        let s_14_10: Bits = Bits::new(s_14_7 as u128, 64u16);
        // C s_14_11: const #63s : i
        let s_14_11: i128 = 63;
        // C s_14_12: const #1u : u64
        let s_14_12: u64 = 1;
        // C s_14_13: cast zx s_14_12 -> bv
        let s_14_13: Bits = Bits::new(s_14_12 as u128, 64u16);
        // C s_14_14: lsl s_14_13 s_14_11
        let s_14_14: Bits = s_14_13 << s_14_11;
        // C s_14_15: sub s_14_14 s_14_13
        let s_14_15: Bits = ((s_14_14) - (s_14_13));
        // D s_14_16: and s_14_10 s_14_15
        let s_14_16: Bits = ((s_14_10) & (s_14_15));
        // D s_14_17: lsl s_14_16 s_14_8
        let s_14_17: Bits = s_14_16 << s_14_8;
        // C s_14_18: lsl s_14_15 s_14_8
        let s_14_18: Bits = s_14_15 << s_14_8;
        // C s_14_19: cmpl s_14_18
        let s_14_19: Bits = !s_14_18;
        // D s_14_20: and s_14_9 s_14_19
        let s_14_20: Bits = ((s_14_9) & (s_14_19));
        // D s_14_21: or s_14_20 s_14_17
        let s_14_21: Bits = ((s_14_20) | (s_14_17));
        // D s_14_22: cast reint s_14_21 -> u128
        let s_14_22: u128 = (s_14_21.value() as u128);
        // D s_14_23: call Mk_PAR_EL1_Type(s_14_22)
        let s_14_23: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_14_22,
        );
        // D s_14_24: call PAR_EL1_write(s_14_23)
        let s_14_24: () = PAR_EL1_write(state, tracer, s_14_23);
        // N s_14_25: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #24u : u8
        let s_15_0: u8 = 24;
        // C s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 8u16);
        // C s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (s_15_1.value() as i128);
        // C s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #432u : u32
        let s_15_5: u32 = 432;
        // D s_15_6: read-reg s_15_5:u8
        let s_15_6: u8 = {
            let value = state.read_register::<u8>(s_15_5 as isize);
            tracer.read_register(s_15_5 as isize, value);
            value
        };
        // D s_15_7: call AArch64_SystemAccessTrap(s_15_6, s_15_4)
        let s_15_7: () = AArch64_SystemAccessTrap(state, tracer, s_15_6, s_15_4);
        // N s_15_8: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var __HFGWTR_EL2_PAR_EL1:u8
        let s_16_0: bool = fn_state.u__HFGWTR_EL2_PAR_EL1;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var gs#87104 <= s_16_4
        fn_state.gs_87104 = s_16_4;
        // N s_16_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // D s_17_4: not s_17_3
        let s_17_4: bool = !s_17_3;
        // N s_17_5: branch s_17_4 b20 b18
        if s_17_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __SCR_EL3_FGTEn:u8
        let s_18_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#87102 <= s_18_4
        fn_state.gs_87102 = s_18_4;
        // N s_18_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#87102:u8
        let s_19_0: bool = fn_state.gs_87102;
        // D s_19_1: write-var gs#87103 <= s_19_0
        fn_state.gs_87103 = s_19_0;
        // N s_19_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#87102 <= s_20_0
        fn_state.gs_87102 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #146u : u32
        let s_21_0: u32 = 146;
        // S s_21_1: call IsFeatureImplemented(s_21_0)
        let s_21_1: bool = IsFeatureImplemented(state, tracer, s_21_0);
        // D s_21_2: write-var gs#87101 <= s_21_1
        fn_state.gs_87101 = s_21_1;
        // N s_21_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
}
