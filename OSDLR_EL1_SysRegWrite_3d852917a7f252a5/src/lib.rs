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
use u_get_HDFGWTR_EL2_Type_OSDLR_EL1::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use Halted::*;
use IsFeatureImplemented::*;
use X_read::*;
use u_get_MDCR_EL2_Type_TDE::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL3_Type_TDOSA::*;
use Mk_OSDLR_EL1_Type::*;
use u_get_MDCR_EL2_Type_TDOSA::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn OSDLR_EL1_SysRegWrite_3d852917a7f252a5<T: Tracer>(
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
        gs_86966: bool,
        gs_86958: bool,
        gs_86977: bool,
        gs_86964: bool,
        gs_86970: bool,
        gs_86978: bool,
        gs_86973: bool,
        gs_86981: bool,
        gs_86979: bool,
        gs_86967: bool,
        gs_86959: bool,
        gs_86960: bool,
        gs_86965: bool,
        gs_86961: bool,
        gs_86956: bool,
        gs_86976: bool,
        gs_86954: bool,
        gs_86975: bool,
        gs_86968: bool,
        gs_86971: bool,
        gs_86955: bool,
        gs_86980: bool,
        gs_86962: bool,
        gs_86972: bool,
        gs_86969: bool,
        gs_86974: bool,
        gs_86957: bool,
        u__PSTATE_EL: u8,
        u__HDFGWTR_EL2_OSDLR_EL1: bool,
        gs_86983: bool,
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
        // C s_0_3: const #17360u : u32
        let s_0_3: u32 = 17360;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HDFGWTR_EL2_Type_OSDLR_EL1(s_0_4)
        let s_0_5: bool = u_get_HDFGWTR_EL2_Type_OSDLR_EL1(state, tracer, s_0_4);
        // D s_0_6: write-var __HDFGWTR_EL2_OSDLR_EL1 <= s_0_5
        fn_state.u__HDFGWTR_EL2_OSDLR_EL1 = s_0_5;
        // C s_0_7: const #104880u : u32
        let s_0_7: u32 = 104880;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL2_Type_TDOSA(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL2_Type_TDOSA(state, tracer, s_0_8);
        // D s_0_10: read-var __PSTATE_EL:u8
        let s_0_10: u8 = fn_state.u__PSTATE_EL;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // C s_0_12: const #448u : u32
        let s_0_12: u32 = 448;
        // D s_0_13: read-reg s_0_12:u8
        let s_0_13: u8 = {
            let value = state.read_register::<u8>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // D s_0_14: cast zx s_0_13 -> bv
        let s_0_14: Bits = Bits::new(s_0_13 as u128, 2u16);
        // D s_0_15: cmp-eq s_0_11 s_0_14
        let s_0_15: bool = ((s_0_11) == (s_0_14));
        // N s_0_16: branch s_0_15 b108 b1
        if s_0_15 {
            return block_108(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b43 b2
        if s_1_5 {
            return block_43(state, tracer, fn_state);
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
        // D s_5_4: call Mk_OSDLR_EL1_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_OSDLR_EL1_Type(state, tracer, s_5_3);
        // C s_5_5: const #102856u : u32
        let s_5_5: u32 = 102856;
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
        // N s_6_2: branch s_6_1 b42 b7
        if s_6_1 {
            return block_42(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#86954 <= s_7_0
        fn_state.gs_86954 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#86954:u8
        let s_8_0: bool = fn_state.gs_86954;
        // N s_8_1: branch s_8_0 b41 b9
        if s_8_0 {
            return block_41(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#86955 <= s_9_0
        fn_state.gs_86955 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#86955:u8
        let s_10_0: bool = fn_state.gs_86955;
        // N s_10_1: branch s_10_0 b40 b11
        if s_10_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#86956 <= s_11_0
        fn_state.gs_86956 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#86956:u8
        let s_12_0: bool = fn_state.gs_86956;
        // N s_12_1: branch s_12_0 b39 b13
        if s_12_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#86957 <= s_13_0
        fn_state.gs_86957 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#86957:u8
        let s_14_0: bool = fn_state.gs_86957;
        // N s_14_1: branch s_14_0 b35 b15
        if s_14_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#86959 <= s_15_0
        fn_state.gs_86959 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#86959:u8
        let s_16_0: bool = fn_state.gs_86959;
        // N s_16_1: branch s_16_0 b34 b17
        if s_16_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
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
        // N s_17_4: branch s_17_3 b33 b18
        if s_17_3 {
            return block_33(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#86960 <= s_18_0
        fn_state.gs_86960 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#86960:u8
        let s_19_0: bool = fn_state.gs_86960;
        // N s_19_1: branch s_19_0 b29 b20
        if s_19_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#86962 <= s_20_0
        fn_state.gs_86962 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#86962:u8
        let s_21_0: bool = fn_state.gs_86962;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #64s : i64
        let s_22_0: i64 = 64;
        // D s_22_1: read-var t:i
        let s_22_1: i128 = fn_state.t;
        // D s_22_2: call X_read(s_22_1, s_22_0)
        let s_22_2: Bits = X_read(state, tracer, s_22_1, s_22_0);
        // D s_22_3: cast reint s_22_2 -> u64
        let s_22_3: u64 = (s_22_2.value() as u64);
        // D s_22_4: call Mk_OSDLR_EL1_Type(s_22_3)
        let s_22_4: ProductType5c790c8ef59cc8b2 = Mk_OSDLR_EL1_Type(
            state,
            tracer,
            s_22_3,
        );
        // C s_22_5: const #102856u : u32
        let s_22_5: u32 = 102856;
        // N s_22_6: write-reg s_22_5 <= s_22_4
        let s_22_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_22_5 as isize, s_22_4);
            tracer.write_register(s_22_5 as isize, s_22_4);
        };
        // N s_22_7: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call Halted(s_23_0)
        let s_23_1: bool = Halted(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b28 b24
        if s_23_1 {
            return block_28(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#86964 <= s_24_0
        fn_state.gs_86964 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#86964:u8
        let s_25_0: bool = fn_state.gs_86964;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #24u : u8
        let s_26_0: u8 = 24;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #424u : u32
        let s_26_5: u32 = 424;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EDSCR_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_28_0);
        // S s_28_2: call _get_EDSCR_Type_SDD(s_28_1)
        let s_28_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_28_1);
        // S s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // S s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#86964 <= s_28_6
        fn_state.gs_86964 = s_28_6;
        // N s_28_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #18u : u32
        let s_29_0: u32 = 18;
        // S s_29_1: call IsFeatureImplemented(s_29_0)
        let s_29_1: bool = IsFeatureImplemented(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b32 b30
        if s_29_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_30_0: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // S s_30_1: call __IMPDEF_boolean(s_30_0)
        let s_30_1: bool = u__IMPDEF_boolean(state, tracer, s_30_0);
        // D s_30_2: write-var gs#86961 <= s_30_1
        fn_state.gs_86961 = s_30_1;
        // N s_30_3: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#86961:u8
        let s_31_0: bool = fn_state.gs_86961;
        // D s_31_1: write-var gs#86962 <= s_31_0
        fn_state.gs_86962 = s_31_0;
        // N s_31_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#86961 <= s_32_0
        fn_state.gs_86961 = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #22712u : u32
        let s_33_0: u32 = 22712;
        // D s_33_1: read-reg s_33_0:struct
        let s_33_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call _get_MDCR_EL3_Type_TDOSA(s_33_1)
        let s_33_2: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_33_1);
        // D s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #1u : u8
        let s_33_4: bool = true;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // D s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // D s_33_7: write-var gs#86960 <= s_33_6
        fn_state.gs_86960 = s_33_6;
        // N s_33_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #18u : u32
        let s_35_0: u32 = 18;
        // S s_35_1: call IsFeatureImplemented(s_35_0)
        let s_35_1: bool = IsFeatureImplemented(state, tracer, s_35_0);
        // N s_35_2: branch s_35_1 b38 b36
        if s_35_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_36_0: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // S s_36_1: call __IMPDEF_boolean(s_36_0)
        let s_36_1: bool = u__IMPDEF_boolean(state, tracer, s_36_0);
        // D s_36_2: write-var gs#86958 <= s_36_1
        fn_state.gs_86958 = s_36_1;
        // N s_36_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#86958:u8
        let s_37_0: bool = fn_state.gs_86958;
        // D s_37_1: write-var gs#86959 <= s_37_0
        fn_state.gs_86959 = s_37_0;
        // N s_37_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#86958 <= s_38_0
        fn_state.gs_86958 = s_38_0;
        // N s_38_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #22712u : u32
        let s_39_0: u32 = 22712;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_MDCR_EL3_Type_TDOSA(s_39_1)
        let s_39_2: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_39_1);
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#86957 <= s_39_6
        fn_state.gs_86957 = s_39_6;
        // N s_39_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_40_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_40_1: call __IMPDEF_boolean(s_40_0)
        let s_40_1: bool = u__IMPDEF_boolean(state, tracer, s_40_0);
        // D s_40_2: write-var gs#86956 <= s_40_1
        fn_state.gs_86956 = s_40_1;
        // N s_40_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EDSCR_read(s_41_0)
        let s_41_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_41_0);
        // S s_41_2: call _get_EDSCR_Type_SDD(s_41_1)
        let s_41_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_41_1);
        // S s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #1u : u8
        let s_41_4: bool = true;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // S s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // D s_41_7: write-var gs#86955 <= s_41_6
        fn_state.gs_86955 = s_41_6;
        // N s_41_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #424u : u32
        let s_42_0: u32 = 424;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // C s_42_2: const #2u : u8
        let s_42_2: u8 = 2;
        // D s_42_3: cmp-lt s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) < (s_42_2));
        // D s_42_4: write-var gs#86954 <= s_42_3
        fn_state.gs_86954 = s_42_3;
        // N s_42_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call Halted(s_43_0)
        let s_43_1: bool = Halted(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b107 b44
        if s_43_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#86965 <= s_44_0
        fn_state.gs_86965 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#86965:u8
        let s_45_0: bool = fn_state.gs_86965;
        // N s_45_1: branch s_45_0 b106 b46
        if s_45_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#86966 <= s_46_0
        fn_state.gs_86966 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#86966:u8
        let s_47_0: bool = fn_state.gs_86966;
        // N s_47_1: branch s_47_0 b105 b48
        if s_47_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#86967 <= s_48_0
        fn_state.gs_86967 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#86967:u8
        let s_49_0: bool = fn_state.gs_86967;
        // N s_49_1: branch s_49_0 b104 b50
        if s_49_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#86968 <= s_50_0
        fn_state.gs_86968 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#86968:u8
        let s_51_0: bool = fn_state.gs_86968;
        // N s_51_1: branch s_51_0 b100 b52
        if s_51_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#86970 <= s_52_0
        fn_state.gs_86970 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#86970:u8
        let s_53_0: bool = fn_state.gs_86970;
        // N s_53_1: branch s_53_0 b99 b54
        if s_53_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EL2Enabled(s_54_0)
        let s_54_1: bool = EL2Enabled(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b98 b55
        if s_54_1 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#86971 <= s_55_0
        fn_state.gs_86971 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#86971:u8
        let s_56_0: bool = fn_state.gs_86971;
        // N s_56_1: branch s_56_0 b94 b57
        if s_56_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#86973 <= s_57_0
        fn_state.gs_86973 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#86973:u8
        let s_58_0: bool = fn_state.gs_86973;
        // N s_58_1: branch s_58_0 b93 b59
        if s_58_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#86974 <= s_59_0
        fn_state.gs_86974 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#86974:u8
        let s_60_0: bool = fn_state.gs_86974;
        // N s_60_1: branch s_60_0 b92 b61
        if s_60_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#86975 <= s_61_0
        fn_state.gs_86975 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#86975:u8
        let s_62_0: bool = fn_state.gs_86975;
        // N s_62_1: branch s_62_0 b91 b63
        if s_62_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EL2Enabled(s_63_0)
        let s_63_1: bool = EL2Enabled(state, tracer, s_63_0);
        // N s_63_2: branch s_63_1 b90 b64
        if s_63_1 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#86976 <= s_64_0
        fn_state.gs_86976 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#86976:u8
        let s_65_0: bool = fn_state.gs_86976;
        // N s_65_1: branch s_65_0 b86 b66
        if s_65_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#86978 <= s_66_0
        fn_state.gs_86978 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#86978:u8
        let s_67_0: bool = fn_state.gs_86978;
        // N s_67_1: branch s_67_0 b85 b68
        if s_67_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #424u : u32
        let s_68_0: u32 = 424;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // C s_68_2: const #2u : u8
        let s_68_2: u8 = 2;
        // D s_68_3: cmp-lt s_68_1 s_68_2
        let s_68_3: bool = ((s_68_1) < (s_68_2));
        // N s_68_4: branch s_68_3 b84 b69
        if s_68_3 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#86979 <= s_69_0
        fn_state.gs_86979 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#86979:u8
        let s_70_0: bool = fn_state.gs_86979;
        // N s_70_1: branch s_70_0 b80 b71
        if s_70_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#86981 <= s_71_0
        fn_state.gs_86981 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#86981:u8
        let s_72_0: bool = fn_state.gs_86981;
        // N s_72_1: branch s_72_0 b74 b73
        if s_72_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #64s : i64
        let s_73_0: i64 = 64;
        // D s_73_1: read-var t:i
        let s_73_1: i128 = fn_state.t;
        // D s_73_2: call X_read(s_73_1, s_73_0)
        let s_73_2: Bits = X_read(state, tracer, s_73_1, s_73_0);
        // D s_73_3: cast reint s_73_2 -> u64
        let s_73_3: u64 = (s_73_2.value() as u64);
        // D s_73_4: call Mk_OSDLR_EL1_Type(s_73_3)
        let s_73_4: ProductType5c790c8ef59cc8b2 = Mk_OSDLR_EL1_Type(
            state,
            tracer,
            s_73_3,
        );
        // C s_73_5: const #102856u : u32
        let s_73_5: u32 = 102856;
        // N s_73_6: write-reg s_73_5 <= s_73_4
        let s_73_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_73_5 as isize, s_73_4);
            tracer.write_register(s_73_5 as isize, s_73_4);
        };
        // N s_73_7: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call Halted(s_74_0)
        let s_74_1: bool = Halted(state, tracer, s_74_0);
        // N s_74_2: branch s_74_1 b79 b75
        if s_74_1 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#86983 <= s_75_0
        fn_state.gs_86983 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#86983:u8
        let s_76_0: bool = fn_state.gs_86983;
        // N s_76_1: branch s_76_0 b78 b77
        if s_76_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
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
        // C s_77_5: const #424u : u32
        let s_77_5: u32 = 424;
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
        // N s_78_0: panic
        panic!("{:?}", ());
        // N s_78_1: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EDSCR_read(s_79_0)
        let s_79_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_79_0);
        // S s_79_2: call _get_EDSCR_Type_SDD(s_79_1)
        let s_79_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_79_1);
        // S s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // C s_79_4: const #1u : u8
        let s_79_4: bool = true;
        // C s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 1u16);
        // S s_79_6: cmp-eq s_79_3 s_79_5
        let s_79_6: bool = ((s_79_3) == (s_79_5));
        // D s_79_7: write-var gs#86983 <= s_79_6
        fn_state.gs_86983 = s_79_6;
        // N s_79_8: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #18u : u32
        let s_80_0: u32 = 18;
        // S s_80_1: call IsFeatureImplemented(s_80_0)
        let s_80_1: bool = IsFeatureImplemented(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b83 b81
        if s_80_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_81_0: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // S s_81_1: call __IMPDEF_boolean(s_81_0)
        let s_81_1: bool = u__IMPDEF_boolean(state, tracer, s_81_0);
        // D s_81_2: write-var gs#86980 <= s_81_1
        fn_state.gs_86980 = s_81_1;
        // N s_81_3: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#86980:u8
        let s_82_0: bool = fn_state.gs_86980;
        // D s_82_1: write-var gs#86981 <= s_82_0
        fn_state.gs_86981 = s_82_0;
        // N s_82_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // D s_83_1: write-var gs#86980 <= s_83_0
        fn_state.gs_86980 = s_83_0;
        // N s_83_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #22712u : u32
        let s_84_0: u32 = 22712;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call _get_MDCR_EL3_Type_TDOSA(s_84_1)
        let s_84_2: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_84_1);
        // D s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // C s_84_4: const #1u : u8
        let s_84_4: bool = true;
        // C s_84_5: cast zx s_84_4 -> bv
        let s_84_5: Bits = Bits::new(s_84_4 as u128, 1u16);
        // D s_84_6: cmp-eq s_84_3 s_84_5
        let s_84_6: bool = ((s_84_3) == (s_84_5));
        // D s_84_7: write-var gs#86979 <= s_84_6
        fn_state.gs_86979 = s_84_6;
        // N s_84_8: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #24u : u8
        let s_85_0: u8 = 24;
        // C s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 8u16);
        // C s_85_2: cast zx s_85_1 -> i
        let s_85_2: i128 = (s_85_1.value() as i128);
        // C s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (i128::try_from(s_85_3).unwrap());
        // C s_85_5: const #432u : u32
        let s_85_5: u32 = 432;
        // D s_85_6: read-reg s_85_5:u8
        let s_85_6: u8 = {
            let value = state.read_register::<u8>(s_85_5 as isize);
            tracer.read_register(s_85_5 as isize, value);
            value
        };
        // D s_85_7: call AArch64_SystemAccessTrap(s_85_6, s_85_4)
        let s_85_7: () = AArch64_SystemAccessTrap(state, tracer, s_85_6, s_85_4);
        // N s_85_8: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #18u : u32
        let s_86_0: u32 = 18;
        // S s_86_1: call IsFeatureImplemented(s_86_0)
        let s_86_1: bool = IsFeatureImplemented(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b89 b87
        if s_86_1 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #"Trapped by MDCR_EL2.TDOSA" : str
        let s_87_0: &'static str = "Trapped by MDCR_EL2.TDOSA";
        // S s_87_1: call __IMPDEF_boolean(s_87_0)
        let s_87_1: bool = u__IMPDEF_boolean(state, tracer, s_87_0);
        // D s_87_2: write-var gs#86977 <= s_87_1
        fn_state.gs_86977 = s_87_1;
        // N s_87_3: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#86977:u8
        let s_88_0: bool = fn_state.gs_86977;
        // D s_88_1: write-var gs#86978 <= s_88_0
        fn_state.gs_86978 = s_88_0;
        // N s_88_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #1u : u8
        let s_89_0: bool = true;
        // D s_89_1: write-var gs#86977 <= s_89_0
        fn_state.gs_86977 = s_89_0;
        // N s_89_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #104880u : u32
        let s_90_0: u32 = 104880;
        // D s_90_1: read-reg s_90_0:struct
        let s_90_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call _get_MDCR_EL2_Type_TDE(s_90_1)
        let s_90_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_90_1);
        // C s_90_3: const #104880u : u32
        let s_90_3: u32 = 104880;
        // D s_90_4: read-reg s_90_3:struct
        let s_90_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_90_3 as isize);
            tracer.read_register(s_90_3 as isize, value);
            value
        };
        // D s_90_5: call _get_MDCR_EL2_Type_TDOSA(s_90_4)
        let s_90_5: bool = u_get_MDCR_EL2_Type_TDOSA(state, tracer, s_90_4);
        // D s_90_6: cast zx s_90_2 -> bv
        let s_90_6: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_7: cast zx s_90_5 -> bv
        let s_90_7: Bits = Bits::new(s_90_5 as u128, 1u16);
        // D s_90_8: cast reint s_90_6 -> u128
        let s_90_8: u128 = (s_90_6.value() as u128);
        // D s_90_9: size-of s_90_6
        let s_90_9: u16 = s_90_6.length();
        // D s_90_10: cast reint s_90_7 -> u128
        let s_90_10: u128 = (s_90_7.value() as u128);
        // D s_90_11: size-of s_90_7
        let s_90_11: u16 = s_90_7.length();
        // D s_90_12: lsl s_90_8 s_90_11
        let s_90_12: u128 = s_90_8 << s_90_11;
        // D s_90_13: or s_90_12 s_90_10
        let s_90_13: u128 = ((s_90_12) | (s_90_10));
        // D s_90_14: add s_90_9 s_90_11
        let s_90_14: u16 = (s_90_9 + s_90_11);
        // D s_90_15: create-bits s_90_13 s_90_14
        let s_90_15: Bits = Bits::new(s_90_13, s_90_14);
        // D s_90_16: cast reint s_90_15 -> u8
        let s_90_16: u8 = (s_90_15.value() as u8);
        // D s_90_17: cast zx s_90_16 -> bv
        let s_90_17: Bits = Bits::new(s_90_16 as u128, 2u16);
        // C s_90_18: const #0u : u8
        let s_90_18: u8 = 0;
        // C s_90_19: cast zx s_90_18 -> bv
        let s_90_19: Bits = Bits::new(s_90_18 as u128, 2u16);
        // D s_90_20: cmp-ne s_90_17 s_90_19
        let s_90_20: bool = ((s_90_17) != (s_90_19));
        // D s_90_21: write-var gs#86976 <= s_90_20
        fn_state.gs_86976 = s_90_20;
        // N s_90_22: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #24u : u8
        let s_91_0: u8 = 24;
        // C s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 8u16);
        // C s_91_2: cast zx s_91_1 -> i
        let s_91_2: i128 = (s_91_1.value() as i128);
        // C s_91_3: cast reint s_91_2 -> i64
        let s_91_3: i64 = (s_91_2 as i64);
        // C s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (i128::try_from(s_91_3).unwrap());
        // C s_91_5: const #432u : u32
        let s_91_5: u32 = 432;
        // D s_91_6: read-reg s_91_5:u8
        let s_91_6: u8 = {
            let value = state.read_register::<u8>(s_91_5 as isize);
            tracer.read_register(s_91_5 as isize, value);
            value
        };
        // D s_91_7: call AArch64_SystemAccessTrap(s_91_6, s_91_4)
        let s_91_7: () = AArch64_SystemAccessTrap(state, tracer, s_91_6, s_91_4);
        // N s_91_8: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __HDFGWTR_EL2_OSDLR_EL1:u8
        let s_92_0: bool = fn_state.u__HDFGWTR_EL2_OSDLR_EL1;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#86975 <= s_92_4
        fn_state.gs_86975 = s_92_4;
        // N s_92_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #18u : u32
        let s_93_0: u32 = 18;
        // S s_93_1: call IsFeatureImplemented(s_93_0)
        let s_93_1: bool = IsFeatureImplemented(state, tracer, s_93_0);
        // D s_93_2: write-var gs#86974 <= s_93_1
        fn_state.gs_86974 = s_93_1;
        // N s_93_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #424u : u32
        let s_94_0: u32 = 424;
        // D s_94_1: read-reg s_94_0:u8
        let s_94_1: u8 = {
            let value = state.read_register::<u8>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // C s_94_2: const #2u : u8
        let s_94_2: u8 = 2;
        // D s_94_3: cmp-lt s_94_1 s_94_2
        let s_94_3: bool = ((s_94_1) < (s_94_2));
        // D s_94_4: not s_94_3
        let s_94_4: bool = !s_94_3;
        // N s_94_5: branch s_94_4 b97 b95
        if s_94_4 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #90704u : u32
        let s_95_0: u32 = 90704;
        // D s_95_1: read-reg s_95_0:struct
        let s_95_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call _get_SCR_EL3_Type_FGTEn(s_95_1)
        let s_95_2: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_95_1);
        // D s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // C s_95_4: const #1u : u8
        let s_95_4: bool = true;
        // C s_95_5: cast zx s_95_4 -> bv
        let s_95_5: Bits = Bits::new(s_95_4 as u128, 1u16);
        // D s_95_6: cmp-eq s_95_3 s_95_5
        let s_95_6: bool = ((s_95_3) == (s_95_5));
        // D s_95_7: write-var gs#86972 <= s_95_6
        fn_state.gs_86972 = s_95_6;
        // N s_95_8: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#86972:u8
        let s_96_0: bool = fn_state.gs_86972;
        // D s_96_1: write-var gs#86973 <= s_96_0
        fn_state.gs_86973 = s_96_0;
        // N s_96_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #1u : u8
        let s_97_0: bool = true;
        // D s_97_1: write-var gs#86972 <= s_97_0
        fn_state.gs_86972 = s_97_0;
        // N s_97_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #146u : u32
        let s_98_0: u32 = 146;
        // S s_98_1: call IsFeatureImplemented(s_98_0)
        let s_98_1: bool = IsFeatureImplemented(state, tracer, s_98_0);
        // D s_98_2: write-var gs#86971 <= s_98_1
        fn_state.gs_86971 = s_98_1;
        // N s_98_3: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_99_0: panic
        panic!("{:?}", ());
        // N s_99_1: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #18u : u32
        let s_100_0: u32 = 18;
        // S s_100_1: call IsFeatureImplemented(s_100_0)
        let s_100_1: bool = IsFeatureImplemented(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b103 b101
        if s_100_1 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #"Trapped by MDCR_EL3.TDOSA" : str
        let s_101_0: &'static str = "Trapped by MDCR_EL3.TDOSA";
        // S s_101_1: call __IMPDEF_boolean(s_101_0)
        let s_101_1: bool = u__IMPDEF_boolean(state, tracer, s_101_0);
        // D s_101_2: write-var gs#86969 <= s_101_1
        fn_state.gs_86969 = s_101_1;
        // N s_101_3: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#86969:u8
        let s_102_0: bool = fn_state.gs_86969;
        // D s_102_1: write-var gs#86970 <= s_102_0
        fn_state.gs_86970 = s_102_0;
        // N s_102_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #1u : u8
        let s_103_0: bool = true;
        // D s_103_1: write-var gs#86969 <= s_103_0
        fn_state.gs_86969 = s_103_0;
        // N s_103_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #22712u : u32
        let s_104_0: u32 = 22712;
        // D s_104_1: read-reg s_104_0:struct
        let s_104_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: call _get_MDCR_EL3_Type_TDOSA(s_104_1)
        let s_104_2: bool = u_get_MDCR_EL3_Type_TDOSA(state, tracer, s_104_1);
        // D s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 1u16);
        // C s_104_4: const #1u : u8
        let s_104_4: bool = true;
        // C s_104_5: cast zx s_104_4 -> bv
        let s_104_5: Bits = Bits::new(s_104_4 as u128, 1u16);
        // D s_104_6: cmp-eq s_104_3 s_104_5
        let s_104_6: bool = ((s_104_3) == (s_104_5));
        // D s_104_7: write-var gs#86968 <= s_104_6
        fn_state.gs_86968 = s_104_6;
        // N s_104_8: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_105_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_105_1: call __IMPDEF_boolean(s_105_0)
        let s_105_1: bool = u__IMPDEF_boolean(state, tracer, s_105_0);
        // D s_105_2: write-var gs#86967 <= s_105_1
        fn_state.gs_86967 = s_105_1;
        // N s_105_3: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call EDSCR_read(s_106_0)
        let s_106_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_106_0);
        // S s_106_2: call _get_EDSCR_Type_SDD(s_106_1)
        let s_106_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_106_1);
        // S s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // C s_106_4: const #1u : u8
        let s_106_4: bool = true;
        // C s_106_5: cast zx s_106_4 -> bv
        let s_106_5: Bits = Bits::new(s_106_4 as u128, 1u16);
        // S s_106_6: cmp-eq s_106_3 s_106_5
        let s_106_6: bool = ((s_106_3) == (s_106_5));
        // D s_106_7: write-var gs#86966 <= s_106_6
        fn_state.gs_86966 = s_106_6;
        // N s_106_8: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #424u : u32
        let s_107_0: u32 = 424;
        // D s_107_1: read-reg s_107_0:u8
        let s_107_1: u8 = {
            let value = state.read_register::<u8>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // C s_107_2: const #2u : u8
        let s_107_2: u8 = 2;
        // D s_107_3: cmp-lt s_107_1 s_107_2
        let s_107_3: bool = ((s_107_1) < (s_107_2));
        // D s_107_4: write-var gs#86965 <= s_107_3
        fn_state.gs_86965 = s_107_3;
        // N s_107_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_108_0: panic
        panic!("{:?}", ());
        // N s_108_1: return
        return;
    }
}
