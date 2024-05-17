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
use u_get_HCR_EL2_Type_NV::*;
use Halted::*;
use Mk_BRBCR_EL2_Type::*;
use u_get_MDCR_EL3_Type_SBRBE::*;
use u_get_EDSCR_Type_SDD::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_SCR_EL3_Type_NS::*;
use EDSCR_read::*;
use EL2Enabled::*;
use common::*;
pub fn BRBCR_EL2_SysRegWrite_df7df73141024675<T: Tracer>(
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
        gs_81234: bool,
        u__SCR_EL3_NS: bool,
        gs_81228: bool,
        u__EDSCR_SDD: bool,
        gs_81243: bool,
        gs_81252: bool,
        gs_81229: bool,
        gs_81251: bool,
        gs_81227: bool,
        gs_81239: bool,
        gs_81231: bool,
        gs_81253: bool,
        gs_81242: bool,
        u__MDCR_EL3_SBRBE: u8,
        gs_81230: bool,
        gs_81240: bool,
        gs_81248: bool,
        gs_81226: bool,
        gs_81249: bool,
        gs_81241: bool,
        gs_81232: bool,
        u__PSTATE_EL: u8,
        gs_81233: bool,
        u__HCR_EL2_NV: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call EDSCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_EDSCR_Type_SDD(s_0_8)
        let s_0_9: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_8);
        // D s_0_10: write-var __EDSCR_SDD <= s_0_9
        fn_state.u__EDSCR_SDD = s_0_9;
        // C s_0_11: const #22712u : u32
        let s_0_11: u32 = 22712;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL3_Type_SBRBE(s_0_12)
        let s_0_13: u8 = u_get_MDCR_EL3_Type_SBRBE(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL3_SBRBE <= s_0_13
        fn_state.u__MDCR_EL3_SBRBE = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_NS(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_NS <= s_0_17
        fn_state.u__SCR_EL3_NS = s_0_17;
        // D s_0_19: read-var __PSTATE_EL:u8
        let s_0_19: u8 = fn_state.u__PSTATE_EL;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_21: const #448u : u32
        let s_0_21: u32 = 448;
        // D s_0_22: read-reg s_0_21:u8
        let s_0_22: u8 = {
            let value = state.read_register::<u8>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // D s_0_24: cmp-eq s_0_20 s_0_23
        let s_0_24: bool = ((s_0_20) == (s_0_23));
        // N s_0_25: branch s_0_24 b79 b1
        if s_0_24 {
            return block_79(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b73 b2
        if s_1_5 {
            return block_73(state, tracer, fn_state);
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
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: call Mk_BRBCR_EL2_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_BRBCR_EL2_Type(state, tracer, s_5_3);
        // C s_5_5: const #18272u : u32
        let s_5_5: u32 = 18272;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b72 b7
        if s_6_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#81226 <= s_7_0
        fn_state.gs_81226 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#81226:u8
        let s_8_0: bool = fn_state.gs_81226;
        // N s_8_1: branch s_8_0 b71 b9
        if s_8_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#81227 <= s_9_0
        fn_state.gs_81227 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#81227:u8
        let s_10_0: bool = fn_state.gs_81227;
        // N s_10_1: branch s_10_0 b70 b11
        if s_10_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#81228 <= s_11_0
        fn_state.gs_81228 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#81228:u8
        let s_12_0: bool = fn_state.gs_81228;
        // N s_12_1: branch s_12_0 b69 b13
        if s_12_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#81229 <= s_13_0
        fn_state.gs_81229 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#81229:u8
        let s_14_0: bool = fn_state.gs_81229;
        // N s_14_1: branch s_14_0 b68 b15
        if s_14_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#81230 <= s_15_0
        fn_state.gs_81230 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#81230:u8
        let s_16_0: bool = fn_state.gs_81230;
        // N s_16_1: branch s_16_0 b67 b17
        if s_16_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call Halted(s_17_0)
        let s_17_1: bool = Halted(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b66 b18
        if s_17_1 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#81231 <= s_18_0
        fn_state.gs_81231 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#81231:u8
        let s_19_0: bool = fn_state.gs_81231;
        // N s_19_1: branch s_19_0 b65 b20
        if s_19_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#81232 <= s_20_0
        fn_state.gs_81232 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#81232:u8
        let s_21_0: bool = fn_state.gs_81232;
        // N s_21_1: branch s_21_0 b64 b22
        if s_21_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#81233 <= s_22_0
        fn_state.gs_81233 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#81233:u8
        let s_23_0: bool = fn_state.gs_81233;
        // N s_23_1: branch s_23_0 b60 b24
        if s_23_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#81239 <= s_24_0
        fn_state.gs_81239 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#81239:u8
        let s_25_0: bool = fn_state.gs_81239;
        // N s_25_1: branch s_25_0 b59 b26
        if s_25_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#81240 <= s_26_0
        fn_state.gs_81240 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#81240:u8
        let s_27_0: bool = fn_state.gs_81240;
        // N s_27_1: branch s_27_0 b58 b28
        if s_27_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #424u : u32
        let s_28_0: u32 = 424;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // C s_28_2: const #2u : u8
        let s_28_2: u8 = 2;
        // D s_28_3: cmp-lt s_28_1 s_28_2
        let s_28_3: bool = ((s_28_1) < (s_28_2));
        // N s_28_4: branch s_28_3 b57 b29
        if s_28_3 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#81241 <= s_29_0
        fn_state.gs_81241 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#81241:u8
        let s_30_0: bool = fn_state.gs_81241;
        // N s_30_1: branch s_30_0 b56 b31
        if s_30_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#81242 <= s_31_0
        fn_state.gs_81242 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#81242:u8
        let s_32_0: bool = fn_state.gs_81242;
        // N s_32_1: branch s_32_0 b50 b33
        if s_32_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // C s_33_2: const #2u : u8
        let s_33_2: u8 = 2;
        // D s_33_3: cmp-lt s_33_1 s_33_2
        let s_33_3: bool = ((s_33_1) < (s_33_2));
        // N s_33_4: branch s_33_3 b46 b34
        if s_33_3 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#81248 <= s_34_0
        fn_state.gs_81248 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#81248:u8
        let s_35_0: bool = fn_state.gs_81248;
        // N s_35_1: branch s_35_0 b45 b36
        if s_35_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#81249 <= s_36_0
        fn_state.gs_81249 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#81249:u8
        let s_37_0: bool = fn_state.gs_81249;
        // N s_37_1: branch s_37_0 b39 b38
        if s_37_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #64s : i64
        let s_38_0: i64 = 64;
        // D s_38_1: read-var t:i
        let s_38_1: i128 = fn_state.t;
        // D s_38_2: call X_read(s_38_1, s_38_0)
        let s_38_2: Bits = X_read(state, tracer, s_38_1, s_38_0);
        // D s_38_3: cast reint s_38_2 -> u64
        let s_38_3: u64 = (s_38_2.value() as u64);
        // D s_38_4: call Mk_BRBCR_EL2_Type(s_38_3)
        let s_38_4: ProductType5c790c8ef59cc8b2 = Mk_BRBCR_EL2_Type(
            state,
            tracer,
            s_38_3,
        );
        // C s_38_5: const #18272u : u32
        let s_38_5: u32 = 18272;
        // N s_38_6: write-reg s_38_5 <= s_38_4
        let s_38_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_38_5 as isize, s_38_4);
            tracer.write_register(s_38_5 as isize, s_38_4);
        };
        // N s_38_7: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call Halted(s_39_0)
        let s_39_1: bool = Halted(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b44 b40
        if s_39_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#81251 <= s_40_0
        fn_state.gs_81251 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#81251:u8
        let s_41_0: bool = fn_state.gs_81251;
        // N s_41_1: branch s_41_0 b43 b42
        if s_41_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #24u : u8
        let s_42_0: u8 = 24;
        // C s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 8u16);
        // C s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (s_42_1.value() as i128);
        // C s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #424u : u32
        let s_42_5: u32 = 424;
        // D s_42_6: read-reg s_42_5:u8
        let s_42_6: u8 = {
            let value = state.read_register::<u8>(s_42_5 as isize);
            tracer.read_register(s_42_5 as isize, value);
            value
        };
        // D s_42_7: call AArch64_SystemAccessTrap(s_42_6, s_42_4)
        let s_42_7: () = AArch64_SystemAccessTrap(state, tracer, s_42_6, s_42_4);
        // N s_42_8: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: panic
        panic!("{:?}", ());
        // N s_43_1: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __EDSCR_SDD:u8
        let s_44_0: bool = fn_state.u__EDSCR_SDD;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#81251 <= s_44_4
        fn_state.gs_81251 = s_44_4;
        // N s_44_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __SCR_EL3_NS:u8
        let s_45_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #1u : u8
        let s_45_2: bool = true;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#81249 <= s_45_4
        fn_state.gs_81249 = s_45_4;
        // N s_45_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __MDCR_EL3_SBRBE:u8
        let s_46_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_46_1: const #0s : i
        let s_46_1: i128 = 0;
        // D s_46_2: cast zx s_46_0 -> bv
        let s_46_2: Bits = Bits::new(s_46_0 as u128, 2u16);
        // C s_46_3: const #1s : i64
        let s_46_3: i64 = 1;
        // C s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // C s_46_5: const #0s : i
        let s_46_5: i128 = 0;
        // C s_46_6: add s_46_5 s_46_4
        let s_46_6: i128 = (s_46_5 + s_46_4);
        // D s_46_7: bit-extract s_46_2 s_46_1 s_46_6
        let s_46_7: Bits = (Bits::new(
            ((s_46_2) >> (s_46_1)).value(),
            u16::try_from(s_46_6).unwrap(),
        ));
        // D s_46_8: cast reint s_46_7 -> u8
        let s_46_8: bool = ((s_46_7.value()) != 0);
        // D s_46_9: cast zx s_46_8 -> bv
        let s_46_9: Bits = Bits::new(s_46_8 as u128, 1u16);
        // C s_46_10: const #0u : u8
        let s_46_10: bool = false;
        // C s_46_11: cast zx s_46_10 -> bv
        let s_46_11: Bits = Bits::new(s_46_10 as u128, 1u16);
        // D s_46_12: cmp-eq s_46_9 s_46_11
        let s_46_12: bool = ((s_46_9) == (s_46_11));
        // D s_46_13: not s_46_12
        let s_46_13: bool = !s_46_12;
        // N s_46_14: branch s_46_13 b49 b47
        if s_46_13 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#81243 <= s_47_0
        fn_state.gs_81243 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#81243:u8
        let s_48_0: bool = fn_state.gs_81243;
        // D s_48_1: write-var gs#81248 <= s_48_0
        fn_state.gs_81248 = s_48_0;
        // N s_48_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#81243 <= s_49_0
        fn_state.gs_81243 = s_49_0;
        // N s_49_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call Halted(s_50_0)
        let s_50_1: bool = Halted(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b55 b51
        if s_50_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#81252 <= s_51_0
        fn_state.gs_81252 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#81252:u8
        let s_52_0: bool = fn_state.gs_81252;
        // N s_52_1: branch s_52_0 b54 b53
        if s_52_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #24u : u8
        let s_53_0: u8 = 24;
        // C s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 8u16);
        // C s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (s_53_1.value() as i128);
        // C s_53_3: cast reint s_53_2 -> i64
        let s_53_3: i64 = (s_53_2 as i64);
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // C s_53_5: const #424u : u32
        let s_53_5: u32 = 424;
        // D s_53_6: read-reg s_53_5:u8
        let s_53_6: u8 = {
            let value = state.read_register::<u8>(s_53_5 as isize);
            tracer.read_register(s_53_5 as isize, value);
            value
        };
        // D s_53_7: call AArch64_SystemAccessTrap(s_53_6, s_53_4)
        let s_53_7: () = AArch64_SystemAccessTrap(state, tracer, s_53_6, s_53_4);
        // N s_53_8: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: panic
        panic!("{:?}", ());
        // N s_54_1: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __EDSCR_SDD:u8
        let s_55_0: bool = fn_state.u__EDSCR_SDD;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #1u : u8
        let s_55_2: bool = true;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#81252 <= s_55_4
        fn_state.gs_81252 = s_55_4;
        // N s_55_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __SCR_EL3_NS:u8
        let s_56_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #0u : u8
        let s_56_2: bool = false;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#81242 <= s_56_4
        fn_state.gs_81242 = s_56_4;
        // N s_56_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __MDCR_EL3_SBRBE:u8
        let s_57_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 2u16);
        // C s_57_2: const #3u : u8
        let s_57_2: u8 = 3;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 2u16);
        // D s_57_4: cmp-ne s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) != (s_57_3));
        // D s_57_5: write-var gs#81241 <= s_57_4
        fn_state.gs_81241 = s_57_4;
        // N s_57_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: panic
        panic!("{:?}", ());
        // N s_58_1: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var __SCR_EL3_NS:u8
        let s_59_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#81240 <= s_59_4
        fn_state.gs_81240 = s_59_4;
        // N s_59_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __MDCR_EL3_SBRBE:u8
        let s_60_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_60_1: const #0s : i
        let s_60_1: i128 = 0;
        // D s_60_2: cast zx s_60_0 -> bv
        let s_60_2: Bits = Bits::new(s_60_0 as u128, 2u16);
        // C s_60_3: const #1s : i64
        let s_60_3: i64 = 1;
        // C s_60_4: cast zx s_60_3 -> i
        let s_60_4: i128 = (i128::try_from(s_60_3).unwrap());
        // C s_60_5: const #0s : i
        let s_60_5: i128 = 0;
        // C s_60_6: add s_60_5 s_60_4
        let s_60_6: i128 = (s_60_5 + s_60_4);
        // D s_60_7: bit-extract s_60_2 s_60_1 s_60_6
        let s_60_7: Bits = (Bits::new(
            ((s_60_2) >> (s_60_1)).value(),
            u16::try_from(s_60_6).unwrap(),
        ));
        // D s_60_8: cast reint s_60_7 -> u8
        let s_60_8: bool = ((s_60_7.value()) != 0);
        // D s_60_9: cast zx s_60_8 -> bv
        let s_60_9: Bits = Bits::new(s_60_8 as u128, 1u16);
        // C s_60_10: const #0u : u8
        let s_60_10: bool = false;
        // C s_60_11: cast zx s_60_10 -> bv
        let s_60_11: Bits = Bits::new(s_60_10 as u128, 1u16);
        // D s_60_12: cmp-eq s_60_9 s_60_11
        let s_60_12: bool = ((s_60_9) == (s_60_11));
        // D s_60_13: not s_60_12
        let s_60_13: bool = !s_60_12;
        // N s_60_14: branch s_60_13 b63 b61
        if s_60_13 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#81234 <= s_61_0
        fn_state.gs_81234 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#81234:u8
        let s_62_0: bool = fn_state.gs_81234;
        // D s_62_1: write-var gs#81239 <= s_62_0
        fn_state.gs_81239 = s_62_0;
        // N s_62_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#81234 <= s_63_0
        fn_state.gs_81234 = s_63_0;
        // N s_63_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_64_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_64_1: call __IMPDEF_boolean(s_64_0)
        let s_64_1: bool = u__IMPDEF_boolean(state, tracer, s_64_0);
        // D s_64_2: write-var gs#81233 <= s_64_1
        fn_state.gs_81233 = s_64_1;
        // N s_64_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var __EDSCR_SDD:u8
        let s_65_0: bool = fn_state.u__EDSCR_SDD;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #1u : u8
        let s_65_2: bool = true;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#81232 <= s_65_4
        fn_state.gs_81232 = s_65_4;
        // N s_65_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #424u : u32
        let s_66_0: u32 = 424;
        // D s_66_1: read-reg s_66_0:u8
        let s_66_1: u8 = {
            let value = state.read_register::<u8>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // C s_66_2: const #2u : u8
        let s_66_2: u8 = 2;
        // D s_66_3: cmp-lt s_66_1 s_66_2
        let s_66_3: bool = ((s_66_1) < (s_66_2));
        // D s_66_4: write-var gs#81231 <= s_66_3
        fn_state.gs_81231 = s_66_3;
        // N s_66_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: panic
        panic!("{:?}", ());
        // N s_67_1: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __SCR_EL3_NS:u8
        let s_68_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #0u : u8
        let s_68_2: bool = false;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#81230 <= s_68_4
        fn_state.gs_81230 = s_68_4;
        // N s_68_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __MDCR_EL3_SBRBE:u8
        let s_69_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 2u16);
        // C s_69_2: const #3u : u8
        let s_69_2: u8 = 3;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 2u16);
        // D s_69_4: cmp-ne s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) != (s_69_3));
        // D s_69_5: write-var gs#81229 <= s_69_4
        fn_state.gs_81229 = s_69_4;
        // N s_69_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_70_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_70_1: call __IMPDEF_boolean(s_70_0)
        let s_70_1: bool = u__IMPDEF_boolean(state, tracer, s_70_0);
        // D s_70_2: write-var gs#81228 <= s_70_1
        fn_state.gs_81228 = s_70_1;
        // N s_70_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __EDSCR_SDD:u8
        let s_71_0: bool = fn_state.u__EDSCR_SDD;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#81227 <= s_71_4
        fn_state.gs_81227 = s_71_4;
        // N s_71_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #424u : u32
        let s_72_0: u32 = 424;
        // D s_72_1: read-reg s_72_0:u8
        let s_72_1: u8 = {
            let value = state.read_register::<u8>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // C s_72_2: const #2u : u8
        let s_72_2: u8 = 2;
        // D s_72_3: cmp-lt s_72_1 s_72_2
        let s_72_3: bool = ((s_72_1) < (s_72_2));
        // D s_72_4: write-var gs#81226 <= s_72_3
        fn_state.gs_81226 = s_72_3;
        // N s_72_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EL2Enabled(s_73_0)
        let s_73_1: bool = EL2Enabled(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b78 b74
        if s_73_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#81253 <= s_74_0
        fn_state.gs_81253 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#81253:u8
        let s_75_0: bool = fn_state.gs_81253;
        // N s_75_1: branch s_75_0 b77 b76
        if s_75_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: panic
        panic!("{:?}", ());
        // N s_76_1: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #24u : u8
        let s_77_0: u8 = 24;
        // C s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 8u16);
        // C s_77_2: cast zx s_77_1 -> i
        let s_77_2: i128 = (s_77_1.value() as i128);
        // C s_77_3: cast reint s_77_2 -> i64
        let s_77_3: i64 = (s_77_2 as i64);
        // C s_77_4: cast zx s_77_3 -> i
        let s_77_4: i128 = (i128::try_from(s_77_3).unwrap());
        // C s_77_5: const #432u : u32
        let s_77_5: u32 = 432;
        // D s_77_6: read-reg s_77_5:u8
        let s_77_6: u8 = {
            let value = state.read_register::<u8>(s_77_5 as isize);
            tracer.read_register(s_77_5 as isize, value);
            value
        };
        // D s_77_7: call AArch64_SystemAccessTrap(s_77_6, s_77_4)
        let s_77_7: () = AArch64_SystemAccessTrap(state, tracer, s_77_6, s_77_4);
        // N s_77_8: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __HCR_EL2_NV:u8
        let s_78_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#81253 <= s_78_4
        fn_state.gs_81253 = s_78_4;
        // N s_78_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: panic
        panic!("{:?}", ());
        // N s_79_1: return
        return;
    }
}
