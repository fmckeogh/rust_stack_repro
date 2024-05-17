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
use EDSCR_read::*;
use u_get_MDCR_EL2_Type_TDA::*;
use Halted::*;
use HDCR_read::*;
use u_get_HDCR_Type_TDE::*;
use u_get_DBGOSLSR_Type_OSLK::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL2_Type_TDE::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TDA::*;
use R_read::*;
use u_get_MDCR_EL3_Type_TDA::*;
use DBGOSECCR_write::*;
use ELUsingAArch32::*;
use u_get_EDSCR_Type_SDD::*;
use DBGOSLSR_read::*;
use EL2Enabled::*;
use Mk_DBGOSECCR_Type::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn DBGOSECCR_SysRegWrite32_87878830bf235bef<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_126348: bool,
        gs_126344: bool,
        gs_126342: bool,
        gs_126352: bool,
        gs_126349: bool,
        gs_126359: bool,
        u__DBGOSLSR_OSLK: bool,
        gs_126356: bool,
        u__MDCR_EL3_TDA: bool,
        gs_126353: bool,
        gs_126345: bool,
        gs_126351: bool,
        gs_126340: bool,
        gs_126355: bool,
        gs_126343: bool,
        gs_126341: bool,
        gs_126346: bool,
        gs_126347: bool,
        gs_126357: bool,
        gs_126358: bool,
        u__PSTATE_EL: u8,
        gs_126354: bool,
        gs_126350: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_TDA(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TDA(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TDA <= s_0_5
        fn_state.u__MDCR_EL3_TDA = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call DBGOSLSR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = DBGOSLSR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_DBGOSLSR_Type_OSLK(s_0_8)
        let s_0_9: bool = u_get_DBGOSLSR_Type_OSLK(state, tracer, s_0_8);
        // D s_0_10: write-var __DBGOSLSR_OSLK <= s_0_9
        fn_state.u__DBGOSLSR_OSLK = s_0_9;
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
        // N s_0_17: branch s_0_16 b90 b1
        if s_0_16 {
            return block_90(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b41 b2
        if s_1_5 {
            return block_41(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b8 b3
        if s_2_5 {
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
        // D s_5_0: read-var __DBGOSLSR_OSLK:u8
        let s_5_0: bool = fn_state.u__DBGOSLSR_OSLK;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // D s_6_2: call Mk_DBGOSECCR_Type(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = Mk_DBGOSECCR_Type(state, tracer, s_6_1);
        // D s_6_3: call DBGOSECCR_write(s_6_2)
        let s_6_3: () = DBGOSECCR_write(state, tracer, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call Halted(s_8_0)
        let s_8_1: bool = Halted(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b40 b9
        if s_8_1 {
            return block_40(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#126340 <= s_9_0
        fn_state.gs_126340 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#126340:u8
        let s_10_0: bool = fn_state.gs_126340;
        // N s_10_1: branch s_10_0 b39 b11
        if s_10_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#126341 <= s_11_0
        fn_state.gs_126341 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#126341:u8
        let s_12_0: bool = fn_state.gs_126341;
        // N s_12_1: branch s_12_0 b38 b13
        if s_12_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#126342 <= s_13_0
        fn_state.gs_126342 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#126342:u8
        let s_14_0: bool = fn_state.gs_126342;
        // N s_14_1: branch s_14_0 b37 b15
        if s_14_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#126343 <= s_15_0
        fn_state.gs_126343 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#126343:u8
        let s_16_0: bool = fn_state.gs_126343;
        // N s_16_1: branch s_16_0 b36 b17
        if s_16_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#126344 <= s_17_0
        fn_state.gs_126344 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#126344:u8
        let s_18_0: bool = fn_state.gs_126344;
        // N s_18_1: branch s_18_0 b35 b19
        if s_18_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #424u : u32
        let s_19_0: u32 = 424;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // C s_19_2: const #2u : u8
        let s_19_2: u8 = 2;
        // D s_19_3: cmp-lt s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) < (s_19_2));
        // N s_19_4: branch s_19_3 b34 b20
        if s_19_3 {
            return block_34(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#126345 <= s_20_0
        fn_state.gs_126345 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#126345:u8
        let s_21_0: bool = fn_state.gs_126345;
        // N s_21_1: branch s_21_0 b33 b22
        if s_21_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#126346 <= s_22_0
        fn_state.gs_126346 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#126346:u8
        let s_23_0: bool = fn_state.gs_126346;
        // N s_23_1: branch s_23_0 b27 b24
        if s_23_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __DBGOSLSR_OSLK:u8
        let s_24_0: bool = fn_state.u__DBGOSLSR_OSLK;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #0u : u8
        let s_24_2: bool = false;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // N s_24_5: branch s_24_4 b26 b25
        if s_24_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var t:i
        let s_25_0: i128 = fn_state.t;
        // D s_25_1: call R_read(s_25_0)
        let s_25_1: u32 = R_read(state, tracer, s_25_0);
        // D s_25_2: call Mk_DBGOSECCR_Type(s_25_1)
        let s_25_2: ProductType700c18a878c5601b = Mk_DBGOSECCR_Type(
            state,
            tracer,
            s_25_1,
        );
        // D s_25_3: call DBGOSECCR_write(s_25_2)
        let s_25_3: () = DBGOSECCR_write(state, tracer, s_25_2);
        // N s_25_4: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call Halted(s_27_0)
        let s_27_1: bool = Halted(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b32 b28
        if s_27_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#126347 <= s_28_0
        fn_state.gs_126347 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#126347:u8
        let s_29_0: bool = fn_state.gs_126347;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #5u : u8
        let s_30_0: u8 = 5;
        // C s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 8u16);
        // C s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (s_30_1.value() as i128);
        // C s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // C s_30_5: const #424u : u32
        let s_30_5: u32 = 424;
        // D s_30_6: read-reg s_30_5:u8
        let s_30_6: u8 = {
            let value = state.read_register::<u8>(s_30_5 as isize);
            tracer.read_register(s_30_5 as isize, value);
            value
        };
        // D s_30_7: call AArch64_AArch32SystemAccessTrap(s_30_6, s_30_4)
        let s_30_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_30_6, s_30_4);
        // N s_30_8: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EDSCR_read(s_32_0)
        let s_32_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_32_0);
        // S s_32_2: call _get_EDSCR_Type_SDD(s_32_1)
        let s_32_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_32_1);
        // S s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // C s_32_4: const #1u : u8
        let s_32_4: bool = true;
        // C s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 1u16);
        // S s_32_6: cmp-eq s_32_3 s_32_5
        let s_32_6: bool = ((s_32_3) == (s_32_5));
        // D s_32_7: write-var gs#126347 <= s_32_6
        fn_state.gs_126347 = s_32_6;
        // N s_32_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __MDCR_EL3_TDA:u8
        let s_33_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#126346 <= s_33_4
        fn_state.gs_126346 = s_33_4;
        // N s_33_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #424u : u32
        let s_34_0: u32 = 424;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call ELUsingAArch32(s_34_1)
        let s_34_2: bool = ELUsingAArch32(state, tracer, s_34_1);
        // D s_34_3: not s_34_2
        let s_34_3: bool = !s_34_2;
        // D s_34_4: write-var gs#126345 <= s_34_3
        fn_state.gs_126345 = s_34_3;
        // N s_34_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var __MDCR_EL3_TDA:u8
        let s_36_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#126344 <= s_36_4
        fn_state.gs_126344 = s_36_4;
        // N s_36_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #424u : u32
        let s_37_0: u32 = 424;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call ELUsingAArch32(s_37_1)
        let s_37_2: bool = ELUsingAArch32(state, tracer, s_37_1);
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // D s_37_4: write-var gs#126343 <= s_37_3
        fn_state.gs_126343 = s_37_3;
        // N s_37_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_38_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_38_1: call __IMPDEF_boolean(s_38_0)
        let s_38_1: bool = u__IMPDEF_boolean(state, tracer, s_38_0);
        // D s_38_2: write-var gs#126342 <= s_38_1
        fn_state.gs_126342 = s_38_1;
        // N s_38_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EDSCR_read(s_39_0)
        let s_39_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_39_0);
        // S s_39_2: call _get_EDSCR_Type_SDD(s_39_1)
        let s_39_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_39_1);
        // S s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // S s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#126341 <= s_39_6
        fn_state.gs_126341 = s_39_6;
        // N s_39_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #424u : u32
        let s_40_0: u32 = 424;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // C s_40_2: const #2u : u8
        let s_40_2: u8 = 2;
        // D s_40_3: cmp-lt s_40_1 s_40_2
        let s_40_3: bool = ((s_40_1) < (s_40_2));
        // D s_40_4: write-var gs#126340 <= s_40_3
        fn_state.gs_126340 = s_40_3;
        // N s_40_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call Halted(s_41_0)
        let s_41_1: bool = Halted(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b89 b42
        if s_41_1 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#126348 <= s_42_0
        fn_state.gs_126348 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#126348:u8
        let s_43_0: bool = fn_state.gs_126348;
        // N s_43_1: branch s_43_0 b88 b44
        if s_43_0 {
            return block_88(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#126349 <= s_44_0
        fn_state.gs_126349 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#126349:u8
        let s_45_0: bool = fn_state.gs_126349;
        // N s_45_1: branch s_45_0 b87 b46
        if s_45_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#126350 <= s_46_0
        fn_state.gs_126350 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#126350:u8
        let s_47_0: bool = fn_state.gs_126350;
        // N s_47_1: branch s_47_0 b86 b48
        if s_47_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#126351 <= s_48_0
        fn_state.gs_126351 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#126351:u8
        let s_49_0: bool = fn_state.gs_126351;
        // N s_49_1: branch s_49_0 b85 b50
        if s_49_0 {
            return block_85(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#126352 <= s_50_0
        fn_state.gs_126352 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#126352:u8
        let s_51_0: bool = fn_state.gs_126352;
        // N s_51_1: branch s_51_0 b84 b52
        if s_51_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b83 b53
        if s_52_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#126353 <= s_53_0
        fn_state.gs_126353 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#126353:u8
        let s_54_0: bool = fn_state.gs_126353;
        // N s_54_1: branch s_54_0 b82 b55
        if s_54_0 {
            return block_82(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#126354 <= s_55_0
        fn_state.gs_126354 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#126354:u8
        let s_56_0: bool = fn_state.gs_126354;
        // N s_56_1: branch s_56_0 b81 b57
        if s_56_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EL2Enabled(s_57_0)
        let s_57_1: bool = EL2Enabled(state, tracer, s_57_0);
        // N s_57_2: branch s_57_1 b80 b58
        if s_57_1 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#126355 <= s_58_0
        fn_state.gs_126355 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#126355:u8
        let s_59_0: bool = fn_state.gs_126355;
        // N s_59_1: branch s_59_0 b79 b60
        if s_59_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#126356 <= s_60_0
        fn_state.gs_126356 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#126356:u8
        let s_61_0: bool = fn_state.gs_126356;
        // N s_61_1: branch s_61_0 b78 b62
        if s_61_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // C s_62_2: const #2u : u8
        let s_62_2: u8 = 2;
        // D s_62_3: cmp-lt s_62_1 s_62_2
        let s_62_3: bool = ((s_62_1) < (s_62_2));
        // N s_62_4: branch s_62_3 b77 b63
        if s_62_3 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#126357 <= s_63_0
        fn_state.gs_126357 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#126357:u8
        let s_64_0: bool = fn_state.gs_126357;
        // N s_64_1: branch s_64_0 b76 b65
        if s_64_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#126358 <= s_65_0
        fn_state.gs_126358 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#126358:u8
        let s_66_0: bool = fn_state.gs_126358;
        // N s_66_1: branch s_66_0 b70 b67
        if s_66_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __DBGOSLSR_OSLK:u8
        let s_67_0: bool = fn_state.u__DBGOSLSR_OSLK;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #0u : u8
        let s_67_2: bool = false;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // N s_67_5: branch s_67_4 b69 b68
        if s_67_4 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var t:i
        let s_68_0: i128 = fn_state.t;
        // D s_68_1: call R_read(s_68_0)
        let s_68_1: u32 = R_read(state, tracer, s_68_0);
        // D s_68_2: call Mk_DBGOSECCR_Type(s_68_1)
        let s_68_2: ProductType700c18a878c5601b = Mk_DBGOSECCR_Type(
            state,
            tracer,
            s_68_1,
        );
        // D s_68_3: call DBGOSECCR_write(s_68_2)
        let s_68_3: () = DBGOSECCR_write(state, tracer, s_68_2);
        // N s_68_4: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_69_0: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call Halted(s_70_0)
        let s_70_1: bool = Halted(state, tracer, s_70_0);
        // N s_70_2: branch s_70_1 b75 b71
        if s_70_1 {
            return block_75(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#126359 <= s_71_0
        fn_state.gs_126359 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#126359:u8
        let s_72_0: bool = fn_state.gs_126359;
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
        // C s_73_0: const #5u : u8
        let s_73_0: u8 = 5;
        // C s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 8u16);
        // C s_73_2: cast zx s_73_1 -> i
        let s_73_2: i128 = (s_73_1.value() as i128);
        // C s_73_3: cast reint s_73_2 -> i64
        let s_73_3: i64 = (s_73_2 as i64);
        // C s_73_4: cast zx s_73_3 -> i
        let s_73_4: i128 = (i128::try_from(s_73_3).unwrap());
        // C s_73_5: const #424u : u32
        let s_73_5: u32 = 424;
        // D s_73_6: read-reg s_73_5:u8
        let s_73_6: u8 = {
            let value = state.read_register::<u8>(s_73_5 as isize);
            tracer.read_register(s_73_5 as isize, value);
            value
        };
        // D s_73_7: call AArch64_AArch32SystemAccessTrap(s_73_6, s_73_4)
        let s_73_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_73_6, s_73_4);
        // N s_73_8: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: panic
        panic!("{:?}", ());
        // N s_74_1: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call EDSCR_read(s_75_0)
        let s_75_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_75_0);
        // S s_75_2: call _get_EDSCR_Type_SDD(s_75_1)
        let s_75_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_75_1);
        // S s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // C s_75_4: const #1u : u8
        let s_75_4: bool = true;
        // C s_75_5: cast zx s_75_4 -> bv
        let s_75_5: Bits = Bits::new(s_75_4 as u128, 1u16);
        // S s_75_6: cmp-eq s_75_3 s_75_5
        let s_75_6: bool = ((s_75_3) == (s_75_5));
        // D s_75_7: write-var gs#126359 <= s_75_6
        fn_state.gs_126359 = s_75_6;
        // N s_75_8: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __MDCR_EL3_TDA:u8
        let s_76_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#126358 <= s_76_4
        fn_state.gs_126358 = s_76_4;
        // N s_76_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #424u : u32
        let s_77_0: u32 = 424;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call ELUsingAArch32(s_77_1)
        let s_77_2: bool = ELUsingAArch32(state, tracer, s_77_1);
        // D s_77_3: not s_77_2
        let s_77_3: bool = !s_77_2;
        // D s_77_4: write-var gs#126357 <= s_77_3
        fn_state.gs_126357 = s_77_3;
        // N s_77_5: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #5u : u8
        let s_78_0: u8 = 5;
        // C s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 8u16);
        // C s_78_2: cast zx s_78_1 -> i
        let s_78_2: i128 = (s_78_1.value() as i128);
        // C s_78_3: cast reint s_78_2 -> i64
        let s_78_3: i64 = (s_78_2 as i64);
        // C s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (i128::try_from(s_78_3).unwrap());
        // S s_78_5: call AArch32_TakeHypTrapException(s_78_4)
        let s_78_5: () = AArch32_TakeHypTrapException(state, tracer, s_78_4);
        // N s_78_6: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call HDCR_read(s_79_0)
        let s_79_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_79_0);
        // S s_79_2: call _get_HDCR_Type_TDE(s_79_1)
        let s_79_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_79_1);
        // C s_79_3: const #() : ()
        let s_79_3: () = ();
        // S s_79_4: call HDCR_read(s_79_3)
        let s_79_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_79_3);
        // S s_79_5: call _get_HDCR_Type_TDA(s_79_4)
        let s_79_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_79_4);
        // S s_79_6: cast zx s_79_2 -> bv
        let s_79_6: Bits = Bits::new(s_79_2 as u128, 1u16);
        // S s_79_7: cast zx s_79_5 -> bv
        let s_79_7: Bits = Bits::new(s_79_5 as u128, 1u16);
        // S s_79_8: cast reint s_79_6 -> u128
        let s_79_8: u128 = (s_79_6.value() as u128);
        // D s_79_9: size-of s_79_6
        let s_79_9: u16 = s_79_6.length();
        // S s_79_10: cast reint s_79_7 -> u128
        let s_79_10: u128 = (s_79_7.value() as u128);
        // D s_79_11: size-of s_79_7
        let s_79_11: u16 = s_79_7.length();
        // D s_79_12: lsl s_79_8 s_79_11
        let s_79_12: u128 = s_79_8 << s_79_11;
        // D s_79_13: or s_79_12 s_79_10
        let s_79_13: u128 = ((s_79_12) | (s_79_10));
        // D s_79_14: add s_79_9 s_79_11
        let s_79_14: u16 = (s_79_9 + s_79_11);
        // D s_79_15: create-bits s_79_13 s_79_14
        let s_79_15: Bits = Bits::new(s_79_13, s_79_14);
        // D s_79_16: cast reint s_79_15 -> u8
        let s_79_16: u8 = (s_79_15.value() as u8);
        // D s_79_17: cast zx s_79_16 -> bv
        let s_79_17: Bits = Bits::new(s_79_16 as u128, 2u16);
        // C s_79_18: const #0u : u8
        let s_79_18: u8 = 0;
        // C s_79_19: cast zx s_79_18 -> bv
        let s_79_19: Bits = Bits::new(s_79_18 as u128, 2u16);
        // D s_79_20: cmp-ne s_79_17 s_79_19
        let s_79_20: bool = ((s_79_17) != (s_79_19));
        // D s_79_21: write-var gs#126356 <= s_79_20
        fn_state.gs_126356 = s_79_20;
        // N s_79_22: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #432u : u32
        let s_80_0: u32 = 432;
        // D s_80_1: read-reg s_80_0:u8
        let s_80_1: u8 = {
            let value = state.read_register::<u8>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call ELUsingAArch32(s_80_1)
        let s_80_2: bool = ELUsingAArch32(state, tracer, s_80_1);
        // D s_80_3: write-var gs#126355 <= s_80_2
        fn_state.gs_126355 = s_80_2;
        // N s_80_4: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #5u : u8
        let s_81_0: u8 = 5;
        // C s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 8u16);
        // C s_81_2: cast zx s_81_1 -> i
        let s_81_2: i128 = (s_81_1.value() as i128);
        // C s_81_3: cast reint s_81_2 -> i64
        let s_81_3: i64 = (s_81_2 as i64);
        // C s_81_4: cast zx s_81_3 -> i
        let s_81_4: i128 = (i128::try_from(s_81_3).unwrap());
        // C s_81_5: const #432u : u32
        let s_81_5: u32 = 432;
        // D s_81_6: read-reg s_81_5:u8
        let s_81_6: u8 = {
            let value = state.read_register::<u8>(s_81_5 as isize);
            tracer.read_register(s_81_5 as isize, value);
            value
        };
        // D s_81_7: call AArch64_AArch32SystemAccessTrap(s_81_6, s_81_4)
        let s_81_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_81_6, s_81_4);
        // N s_81_8: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #104880u : u32
        let s_82_0: u32 = 104880;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_MDCR_EL2_Type_TDE(s_82_1)
        let s_82_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_82_1);
        // C s_82_3: const #104880u : u32
        let s_82_3: u32 = 104880;
        // D s_82_4: read-reg s_82_3:struct
        let s_82_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_3 as isize);
            tracer.read_register(s_82_3 as isize, value);
            value
        };
        // D s_82_5: call _get_MDCR_EL2_Type_TDA(s_82_4)
        let s_82_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_82_4);
        // D s_82_6: cast zx s_82_2 -> bv
        let s_82_6: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_7: cast zx s_82_5 -> bv
        let s_82_7: Bits = Bits::new(s_82_5 as u128, 1u16);
        // D s_82_8: cast reint s_82_6 -> u128
        let s_82_8: u128 = (s_82_6.value() as u128);
        // D s_82_9: size-of s_82_6
        let s_82_9: u16 = s_82_6.length();
        // D s_82_10: cast reint s_82_7 -> u128
        let s_82_10: u128 = (s_82_7.value() as u128);
        // D s_82_11: size-of s_82_7
        let s_82_11: u16 = s_82_7.length();
        // D s_82_12: lsl s_82_8 s_82_11
        let s_82_12: u128 = s_82_8 << s_82_11;
        // D s_82_13: or s_82_12 s_82_10
        let s_82_13: u128 = ((s_82_12) | (s_82_10));
        // D s_82_14: add s_82_9 s_82_11
        let s_82_14: u16 = (s_82_9 + s_82_11);
        // D s_82_15: create-bits s_82_13 s_82_14
        let s_82_15: Bits = Bits::new(s_82_13, s_82_14);
        // D s_82_16: cast reint s_82_15 -> u8
        let s_82_16: u8 = (s_82_15.value() as u8);
        // D s_82_17: cast zx s_82_16 -> bv
        let s_82_17: Bits = Bits::new(s_82_16 as u128, 2u16);
        // C s_82_18: const #0u : u8
        let s_82_18: u8 = 0;
        // C s_82_19: cast zx s_82_18 -> bv
        let s_82_19: Bits = Bits::new(s_82_18 as u128, 2u16);
        // D s_82_20: cmp-ne s_82_17 s_82_19
        let s_82_20: bool = ((s_82_17) != (s_82_19));
        // D s_82_21: write-var gs#126354 <= s_82_20
        fn_state.gs_126354 = s_82_20;
        // N s_82_22: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #432u : u32
        let s_83_0: u32 = 432;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call ELUsingAArch32(s_83_1)
        let s_83_2: bool = ELUsingAArch32(state, tracer, s_83_1);
        // D s_83_3: not s_83_2
        let s_83_3: bool = !s_83_2;
        // D s_83_4: write-var gs#126353 <= s_83_3
        fn_state.gs_126353 = s_83_3;
        // N s_83_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: panic
        panic!("{:?}", ());
        // N s_84_1: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __MDCR_EL3_TDA:u8
        let s_85_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #1u : u8
        let s_85_2: bool = true;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#126352 <= s_85_4
        fn_state.gs_126352 = s_85_4;
        // N s_85_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #424u : u32
        let s_86_0: u32 = 424;
        // D s_86_1: read-reg s_86_0:u8
        let s_86_1: u8 = {
            let value = state.read_register::<u8>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // D s_86_2: call ELUsingAArch32(s_86_1)
        let s_86_2: bool = ELUsingAArch32(state, tracer, s_86_1);
        // D s_86_3: not s_86_2
        let s_86_3: bool = !s_86_2;
        // D s_86_4: write-var gs#126351 <= s_86_3
        fn_state.gs_126351 = s_86_3;
        // N s_86_5: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_87_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_87_1: call __IMPDEF_boolean(s_87_0)
        let s_87_1: bool = u__IMPDEF_boolean(state, tracer, s_87_0);
        // D s_87_2: write-var gs#126350 <= s_87_1
        fn_state.gs_126350 = s_87_1;
        // N s_87_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call EDSCR_read(s_88_0)
        let s_88_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_88_0);
        // S s_88_2: call _get_EDSCR_Type_SDD(s_88_1)
        let s_88_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_88_1);
        // S s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // C s_88_4: const #1u : u8
        let s_88_4: bool = true;
        // C s_88_5: cast zx s_88_4 -> bv
        let s_88_5: Bits = Bits::new(s_88_4 as u128, 1u16);
        // S s_88_6: cmp-eq s_88_3 s_88_5
        let s_88_6: bool = ((s_88_3) == (s_88_5));
        // D s_88_7: write-var gs#126349 <= s_88_6
        fn_state.gs_126349 = s_88_6;
        // N s_88_8: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #424u : u32
        let s_89_0: u32 = 424;
        // D s_89_1: read-reg s_89_0:u8
        let s_89_1: u8 = {
            let value = state.read_register::<u8>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // C s_89_2: const #2u : u8
        let s_89_2: u8 = 2;
        // D s_89_3: cmp-lt s_89_1 s_89_2
        let s_89_3: bool = ((s_89_1) < (s_89_2));
        // D s_89_4: write-var gs#126348 <= s_89_3
        fn_state.gs_126348 = s_89_3;
        // N s_89_5: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_90_0: panic
        panic!("{:?}", ());
        // N s_90_1: return
        return;
    }
}
