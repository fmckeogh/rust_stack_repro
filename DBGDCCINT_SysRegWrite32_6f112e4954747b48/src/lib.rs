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
use AArch32_TakeHypTrapException::*;
use u_get_MDCR_EL2_Type_TDA::*;
use u_get_HDCR_Type_TDCC::*;
use Halted::*;
use HDCR_read::*;
use u_get_MDCR_EL2_Type_TDCC::*;
use DBGDCCINT_write::*;
use u_get_HDCR_Type_TDE::*;
use u_get_MDCR_EL3_Type_TDCC::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TDA::*;
use u_get_MDCR_EL2_Type_TDE::*;
use ConstrainUnpredictableBool::*;
use R_read::*;
use u_get_MDCR_EL3_Type_TDA::*;
use ELUsingAArch32::*;
use u_get_SDCR_Type_TDCC::*;
use u_get_EDSCR_Type_SDD::*;
use Mk_DBGDCCINT_Type::*;
use EL2Enabled::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn DBGDCCINT_SysRegWrite32_6f112e4954747b48<T: Tracer>(
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
        gs_126121: bool,
        gs_126077: bool,
        gs_126088: bool,
        gs_126110: bool,
        gs_126071: bool,
        gs_126116: bool,
        gs_126106: bool,
        gs_126095: bool,
        gs_126073: bool,
        gs_126085: bool,
        gs_126107: bool,
        u__SDCR_TDCC: bool,
        gs_126099: bool,
        gs_126084: bool,
        gs_126104: bool,
        gs_126117: bool,
        gs_126118: bool,
        gs_126079: bool,
        gs_126078: bool,
        gs_126086: bool,
        gs_126123: bool,
        u__PSTATE_EL: u8,
        gs_126093: bool,
        gs_126089: bool,
        gs_126122: bool,
        gs_126094: bool,
        gs_126082: bool,
        gs_126113: bool,
        gs_126124: bool,
        gs_126100: bool,
        gs_126081: bool,
        gs_126109: bool,
        gs_126074: bool,
        gs_126076: bool,
        gs_126103: bool,
        gs_126101: bool,
        gs_126097: bool,
        gs_126114: bool,
        gs_126102: bool,
        u__MDCR_EL3_TDA: bool,
        gs_126072: bool,
        gs_126126: bool,
        gs_126112: bool,
        gs_126119: bool,
        gs_126080: bool,
        gs_126083: bool,
        gs_126069: bool,
        gs_126111: bool,
        gs_126075: bool,
        gs_126115: bool,
        gs_126090: bool,
        u__MDCR_EL3_TDCC: bool,
        gs_126105: bool,
        u__PSTATE_M: u8,
        gs_126096: bool,
        gs_126070: bool,
        gs_126098: bool,
        u__MDCR_EL2_TDCC: bool,
        gs_126091: bool,
        gs_126092: bool,
        gs_126108: bool,
        u__HDCR_TDCC: bool,
        gs_126125: bool,
        gs_126120: bool,
        gs_126087: bool,
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
        // D s_0_5: call _get_MDCR_EL3_Type_TDCC(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TDCC(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TDCC <= s_0_5
        fn_state.u__MDCR_EL3_TDCC = s_0_5;
        // C s_0_7: const #15048u : u32
        let s_0_7: u32 = 15048;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SDCR_Type_TDCC(s_0_8)
        let s_0_9: bool = u_get_SDCR_Type_TDCC(state, tracer, s_0_8);
        // D s_0_10: write-var __SDCR_TDCC <= s_0_9
        fn_state.u__SDCR_TDCC = s_0_9;
        // C s_0_11: const #22712u : u32
        let s_0_11: u32 = 22712;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL3_Type_TDA(s_0_12)
        let s_0_13: bool = u_get_MDCR_EL3_Type_TDA(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL3_TDA <= s_0_13
        fn_state.u__MDCR_EL3_TDA = s_0_13;
        // C s_0_15: const #104880u : u32
        let s_0_15: u32 = 104880;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_MDCR_EL2_Type_TDCC(s_0_16)
        let s_0_17: bool = u_get_MDCR_EL2_Type_TDCC(state, tracer, s_0_16);
        // D s_0_18: write-var __MDCR_EL2_TDCC <= s_0_17
        fn_state.u__MDCR_EL2_TDCC = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HDCR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HDCR_Type_TDCC(s_0_20)
        let s_0_21: bool = u_get_HDCR_Type_TDCC(state, tracer, s_0_20);
        // D s_0_22: write-var __HDCR_TDCC <= s_0_21
        fn_state.u__HDCR_TDCC = s_0_21;
        // C s_0_23: const #16983u : u32
        let s_0_23: u32 = 16983;
        // D s_0_24: read-reg s_0_23:u8
        let s_0_24: u8 = {
            let value = state.read_register::<u8>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: write-var __PSTATE_M <= s_0_24
        fn_state.u__PSTATE_M = s_0_24;
        // D s_0_26: read-var __PSTATE_EL:u8
        let s_0_26: u8 = fn_state.u__PSTATE_EL;
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // C s_0_28: const #448u : u32
        let s_0_28: u32 = 448;
        // D s_0_29: read-reg s_0_28:u8
        let s_0_29: u8 = {
            let value = state.read_register::<u8>(s_0_28 as isize);
            tracer.read_register(s_0_28 as isize, value);
            value
        };
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 2u16);
        // D s_0_31: cmp-eq s_0_27 s_0_30
        let s_0_31: bool = ((s_0_27) == (s_0_30));
        // N s_0_32: branch s_0_31 b230 b1
        if s_0_31 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call Halted(s_1_0)
        let s_1_1: bool = Halted(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b229 b2
        if s_1_1 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#126069 <= s_2_0
        fn_state.gs_126069 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#126069:u8
        let s_3_0: bool = fn_state.gs_126069;
        // N s_3_1: branch s_3_0 b228 b4
        if s_3_0 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #440u : u32
        let s_4_2: u32 = 440;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b105 b5
        if s_4_5 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var __PSTATE_EL:u8
        let s_5_0: u8 = fn_state.u__PSTATE_EL;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b14 b6
        if s_5_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __PSTATE_EL:u8
        let s_6_0: u8 = fn_state.u__PSTATE_EL;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #424u : u32
        let s_6_2: u32 = 424;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
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
        // D s_8_0: read-var __PSTATE_M:u8
        let s_8_0: u8 = fn_state.u__PSTATE_M;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 5u16);
        // C s_8_2: const #384u : u32
        let s_8_2: u32 = 384;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 5u16);
        // D s_8_5: cmp-ne s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) != (s_8_4));
        // N s_8_6: branch s_8_5 b13 b9
        if s_8_5 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#126070 <= s_9_0
        fn_state.gs_126070 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#126070:u8
        let s_10_0: bool = fn_state.gs_126070;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var t:i
        let s_11_0: i128 = fn_state.t;
        // D s_11_1: call R_read(s_11_0)
        let s_11_1: u32 = R_read(state, tracer, s_11_0);
        // D s_11_2: call Mk_DBGDCCINT_Type(s_11_1)
        let s_11_2: ProductType700c18a878c5601b = Mk_DBGDCCINT_Type(
            state,
            tracer,
            s_11_1,
        );
        // D s_11_3: call DBGDCCINT_write(s_11_2)
        let s_11_3: () = DBGDCCINT_write(state, tracer, s_11_2);
        // N s_11_4: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call AArch32_TakeMonitorTrapException(s_12_0)
        let s_12_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_12_0);
        // N s_12_2: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var __SDCR_TDCC:u8
        let s_13_0: bool = fn_state.u__SDCR_TDCC;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var gs#126070 <= s_13_4
        fn_state.gs_126070 = s_13_4;
        // N s_13_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call Halted(s_14_0)
        let s_14_1: bool = Halted(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b104 b15
        if s_14_1 {
            return block_104(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#126071 <= s_15_0
        fn_state.gs_126071 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#126071:u8
        let s_16_0: bool = fn_state.gs_126071;
        // N s_16_1: branch s_16_0 b103 b17
        if s_16_0 {
            return block_103(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#126072 <= s_17_0
        fn_state.gs_126072 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#126072:u8
        let s_18_0: bool = fn_state.gs_126072;
        // N s_18_1: branch s_18_0 b102 b19
        if s_18_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#126073 <= s_19_0
        fn_state.gs_126073 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#126073:u8
        let s_20_0: bool = fn_state.gs_126073;
        // N s_20_1: branch s_20_0 b101 b21
        if s_20_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#126074 <= s_21_0
        fn_state.gs_126074 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#126074:u8
        let s_22_0: bool = fn_state.gs_126074;
        // N s_22_1: branch s_22_0 b100 b23
        if s_22_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#126075 <= s_23_0
        fn_state.gs_126075 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#126075:u8
        let s_24_0: bool = fn_state.gs_126075;
        // N s_24_1: branch s_24_0 b99 b25
        if s_24_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call Halted(s_25_0)
        let s_25_1: bool = Halted(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b98 b26
        if s_25_1 {
            return block_98(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#126076 <= s_26_0
        fn_state.gs_126076 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#126076:u8
        let s_27_0: bool = fn_state.gs_126076;
        // N s_27_1: branch s_27_0 b97 b28
        if s_27_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#126077 <= s_28_0
        fn_state.gs_126077 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#126077:u8
        let s_29_0: bool = fn_state.gs_126077;
        // N s_29_1: branch s_29_0 b96 b30
        if s_29_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#126078 <= s_30_0
        fn_state.gs_126078 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#126078:u8
        let s_31_0: bool = fn_state.gs_126078;
        // N s_31_1: branch s_31_0 b95 b32
        if s_31_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#126079 <= s_32_0
        fn_state.gs_126079 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#126079:u8
        let s_33_0: bool = fn_state.gs_126079;
        // N s_33_1: branch s_33_0 b94 b34
        if s_33_0 {
            return block_94(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#126080 <= s_34_0
        fn_state.gs_126080 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#126080:u8
        let s_35_0: bool = fn_state.gs_126080;
        // N s_35_1: branch s_35_0 b93 b36
        if s_35_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call Halted(s_36_0)
        let s_36_1: bool = Halted(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b92 b37
        if s_36_1 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#126081 <= s_37_0
        fn_state.gs_126081 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#126081:u8
        let s_38_0: bool = fn_state.gs_126081;
        // N s_38_1: branch s_38_0 b91 b39
        if s_38_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#126082 <= s_39_0
        fn_state.gs_126082 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#126082:u8
        let s_40_0: bool = fn_state.gs_126082;
        // N s_40_1: branch s_40_0 b90 b41
        if s_40_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#126083 <= s_41_0
        fn_state.gs_126083 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#126083:u8
        let s_42_0: bool = fn_state.gs_126083;
        // N s_42_1: branch s_42_0 b89 b43
        if s_42_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#126084 <= s_43_0
        fn_state.gs_126084 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#126084:u8
        let s_44_0: bool = fn_state.gs_126084;
        // N s_44_1: branch s_44_0 b88 b45
        if s_44_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#126085 <= s_45_0
        fn_state.gs_126085 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#126085:u8
        let s_46_0: bool = fn_state.gs_126085;
        // N s_46_1: branch s_46_0 b87 b47
        if s_46_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #424u : u32
        let s_47_0: u32 = 424;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // C s_47_2: const #2u : u8
        let s_47_2: u8 = 2;
        // D s_47_3: cmp-lt s_47_1 s_47_2
        let s_47_3: bool = ((s_47_1) < (s_47_2));
        // N s_47_4: branch s_47_3 b86 b48
        if s_47_3 {
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
        // D s_48_1: write-var gs#126086 <= s_48_0
        fn_state.gs_126086 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#126086:u8
        let s_49_0: bool = fn_state.gs_126086;
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
        // D s_50_1: write-var gs#126087 <= s_50_0
        fn_state.gs_126087 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#126087:u8
        let s_51_0: bool = fn_state.gs_126087;
        // N s_51_1: branch s_51_0 b79 b52
        if s_51_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #424u : u32
        let s_52_0: u32 = 424;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // C s_52_2: const #2u : u8
        let s_52_2: u8 = 2;
        // D s_52_3: cmp-lt s_52_1 s_52_2
        let s_52_3: bool = ((s_52_1) < (s_52_2));
        // N s_52_4: branch s_52_3 b78 b53
        if s_52_3 {
            return block_78(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#126088 <= s_53_0
        fn_state.gs_126088 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#126088:u8
        let s_54_0: bool = fn_state.gs_126088;
        // N s_54_1: branch s_54_0 b77 b55
        if s_54_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#126089 <= s_55_0
        fn_state.gs_126089 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#126089:u8
        let s_56_0: bool = fn_state.gs_126089;
        // N s_56_1: branch s_56_0 b71 b57
        if s_56_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #424u : u32
        let s_57_0: u32 = 424;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // C s_57_2: const #2u : u8
        let s_57_2: u8 = 2;
        // D s_57_3: cmp-lt s_57_1 s_57_2
        let s_57_3: bool = ((s_57_1) < (s_57_2));
        // N s_57_4: branch s_57_3 b70 b58
        if s_57_3 {
            return block_70(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#126090 <= s_58_0
        fn_state.gs_126090 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#126090:u8
        let s_59_0: bool = fn_state.gs_126090;
        // N s_59_1: branch s_59_0 b69 b60
        if s_59_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#126091 <= s_60_0
        fn_state.gs_126091 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#126091:u8
        let s_61_0: bool = fn_state.gs_126091;
        // N s_61_1: branch s_61_0 b63 b62
        if s_61_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var t:i
        let s_62_0: i128 = fn_state.t;
        // D s_62_1: call R_read(s_62_0)
        let s_62_1: u32 = R_read(state, tracer, s_62_0);
        // D s_62_2: call Mk_DBGDCCINT_Type(s_62_1)
        let s_62_2: ProductType700c18a878c5601b = Mk_DBGDCCINT_Type(
            state,
            tracer,
            s_62_1,
        );
        // D s_62_3: call DBGDCCINT_write(s_62_2)
        let s_62_3: () = DBGDCCINT_write(state, tracer, s_62_2);
        // N s_62_4: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call Halted(s_63_0)
        let s_63_1: bool = Halted(state, tracer, s_63_0);
        // N s_63_2: branch s_63_1 b68 b64
        if s_63_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#126092 <= s_64_0
        fn_state.gs_126092 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#126092:u8
        let s_65_0: bool = fn_state.gs_126092;
        // N s_65_1: branch s_65_0 b67 b66
        if s_65_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #5u : u8
        let s_66_0: u8 = 5;
        // C s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 8u16);
        // C s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (s_66_1.value() as i128);
        // C s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // C s_66_5: const #424u : u32
        let s_66_5: u32 = 424;
        // D s_66_6: read-reg s_66_5:u8
        let s_66_6: u8 = {
            let value = state.read_register::<u8>(s_66_5 as isize);
            tracer.read_register(s_66_5 as isize, value);
            value
        };
        // D s_66_7: call AArch64_AArch32SystemAccessTrap(s_66_6, s_66_4)
        let s_66_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_66_6, s_66_4);
        // N s_66_8: return
        return;
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
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call EDSCR_read(s_68_0)
        let s_68_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_68_0);
        // S s_68_2: call _get_EDSCR_Type_SDD(s_68_1)
        let s_68_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_68_1);
        // S s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // C s_68_4: const #1u : u8
        let s_68_4: bool = true;
        // C s_68_5: cast zx s_68_4 -> bv
        let s_68_5: Bits = Bits::new(s_68_4 as u128, 1u16);
        // S s_68_6: cmp-eq s_68_3 s_68_5
        let s_68_6: bool = ((s_68_3) == (s_68_5));
        // D s_68_7: write-var gs#126092 <= s_68_6
        fn_state.gs_126092 = s_68_6;
        // N s_68_8: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __MDCR_EL3_TDA:u8
        let s_69_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#126091 <= s_69_4
        fn_state.gs_126091 = s_69_4;
        // N s_69_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #424u : u32
        let s_70_0: u32 = 424;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call ELUsingAArch32(s_70_1)
        let s_70_2: bool = ELUsingAArch32(state, tracer, s_70_1);
        // D s_70_3: not s_70_2
        let s_70_3: bool = !s_70_2;
        // D s_70_4: write-var gs#126090 <= s_70_3
        fn_state.gs_126090 = s_70_3;
        // N s_70_5: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call Halted(s_71_0)
        let s_71_1: bool = Halted(state, tracer, s_71_0);
        // N s_71_2: branch s_71_1 b76 b72
        if s_71_1 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#126093 <= s_72_0
        fn_state.gs_126093 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#126093:u8
        let s_73_0: bool = fn_state.gs_126093;
        // N s_73_1: branch s_73_0 b75 b74
        if s_73_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call AArch32_TakeMonitorTrapException(s_74_0)
        let s_74_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_74_0);
        // N s_74_2: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_75_0: panic
        panic!("{:?}", ());
        // N s_75_1: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call EDSCR_read(s_76_0)
        let s_76_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_76_0);
        // S s_76_2: call _get_EDSCR_Type_SDD(s_76_1)
        let s_76_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_76_1);
        // S s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // C s_76_4: const #1u : u8
        let s_76_4: bool = true;
        // C s_76_5: cast zx s_76_4 -> bv
        let s_76_5: Bits = Bits::new(s_76_4 as u128, 1u16);
        // S s_76_6: cmp-eq s_76_3 s_76_5
        let s_76_6: bool = ((s_76_3) == (s_76_5));
        // D s_76_7: write-var gs#126093 <= s_76_6
        fn_state.gs_126093 = s_76_6;
        // N s_76_8: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var __SDCR_TDCC:u8
        let s_77_0: bool = fn_state.u__SDCR_TDCC;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#126089 <= s_77_4
        fn_state.gs_126089 = s_77_4;
        // N s_77_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #424u : u32
        let s_78_0: u32 = 424;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call ELUsingAArch32(s_78_1)
        let s_78_2: bool = ELUsingAArch32(state, tracer, s_78_1);
        // D s_78_3: write-var gs#126088 <= s_78_2
        fn_state.gs_126088 = s_78_2;
        // N s_78_4: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call Halted(s_79_0)
        let s_79_1: bool = Halted(state, tracer, s_79_0);
        // N s_79_2: branch s_79_1 b84 b80
        if s_79_1 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#126094 <= s_80_0
        fn_state.gs_126094 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#126094:u8
        let s_81_0: bool = fn_state.gs_126094;
        // N s_81_1: branch s_81_0 b83 b82
        if s_81_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #5u : u8
        let s_82_0: u8 = 5;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // C s_82_5: const #424u : u32
        let s_82_5: u32 = 424;
        // D s_82_6: read-reg s_82_5:u8
        let s_82_6: u8 = {
            let value = state.read_register::<u8>(s_82_5 as isize);
            tracer.read_register(s_82_5 as isize, value);
            value
        };
        // D s_82_7: call AArch64_AArch32SystemAccessTrap(s_82_6, s_82_4)
        let s_82_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_82_6, s_82_4);
        // N s_82_8: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_83_0: panic
        panic!("{:?}", ());
        // N s_83_1: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call EDSCR_read(s_84_0)
        let s_84_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_84_0);
        // S s_84_2: call _get_EDSCR_Type_SDD(s_84_1)
        let s_84_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_84_1);
        // S s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // C s_84_4: const #1u : u8
        let s_84_4: bool = true;
        // C s_84_5: cast zx s_84_4 -> bv
        let s_84_5: Bits = Bits::new(s_84_4 as u128, 1u16);
        // S s_84_6: cmp-eq s_84_3 s_84_5
        let s_84_6: bool = ((s_84_3) == (s_84_5));
        // D s_84_7: write-var gs#126094 <= s_84_6
        fn_state.gs_126094 = s_84_6;
        // N s_84_8: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __MDCR_EL3_TDCC:u8
        let s_85_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #1u : u8
        let s_85_2: bool = true;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#126087 <= s_85_4
        fn_state.gs_126087 = s_85_4;
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
        // D s_86_4: write-var gs#126086 <= s_86_3
        fn_state.gs_126086 = s_86_3;
        // N s_86_5: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_87_0: panic
        panic!("{:?}", ());
        // N s_87_1: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var __MDCR_EL3_TDA:u8
        let s_88_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 1u16);
        // C s_88_2: const #1u : u8
        let s_88_2: bool = true;
        // C s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_4: cmp-eq s_88_1 s_88_3
        let s_88_4: bool = ((s_88_1) == (s_88_3));
        // D s_88_5: write-var gs#126085 <= s_88_4
        fn_state.gs_126085 = s_88_4;
        // N s_88_6: jump b46
        return block_46(state, tracer, fn_state);
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
        // D s_89_2: call ELUsingAArch32(s_89_1)
        let s_89_2: bool = ELUsingAArch32(state, tracer, s_89_1);
        // D s_89_3: not s_89_2
        let s_89_3: bool = !s_89_2;
        // D s_89_4: write-var gs#126084 <= s_89_3
        fn_state.gs_126084 = s_89_3;
        // N s_89_5: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_90_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_90_1: call __IMPDEF_boolean(s_90_0)
        let s_90_1: bool = u__IMPDEF_boolean(state, tracer, s_90_0);
        // D s_90_2: write-var gs#126083 <= s_90_1
        fn_state.gs_126083 = s_90_1;
        // N s_90_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call EDSCR_read(s_91_0)
        let s_91_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_91_0);
        // S s_91_2: call _get_EDSCR_Type_SDD(s_91_1)
        let s_91_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_91_1);
        // S s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // C s_91_4: const #1u : u8
        let s_91_4: bool = true;
        // C s_91_5: cast zx s_91_4 -> bv
        let s_91_5: Bits = Bits::new(s_91_4 as u128, 1u16);
        // S s_91_6: cmp-eq s_91_3 s_91_5
        let s_91_6: bool = ((s_91_3) == (s_91_5));
        // D s_91_7: write-var gs#126082 <= s_91_6
        fn_state.gs_126082 = s_91_6;
        // N s_91_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #424u : u32
        let s_92_0: u32 = 424;
        // D s_92_1: read-reg s_92_0:u8
        let s_92_1: u8 = {
            let value = state.read_register::<u8>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // C s_92_2: const #2u : u8
        let s_92_2: u8 = 2;
        // D s_92_3: cmp-lt s_92_1 s_92_2
        let s_92_3: bool = ((s_92_1) < (s_92_2));
        // D s_92_4: write-var gs#126081 <= s_92_3
        fn_state.gs_126081 = s_92_3;
        // N s_92_5: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_93_0: panic
        panic!("{:?}", ());
        // N s_93_1: return
        return;
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var __SDCR_TDCC:u8
        let s_94_0: bool = fn_state.u__SDCR_TDCC;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #1u : u8
        let s_94_2: bool = true;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: write-var gs#126080 <= s_94_4
        fn_state.gs_126080 = s_94_4;
        // N s_94_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #424u : u32
        let s_95_0: u32 = 424;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call ELUsingAArch32(s_95_1)
        let s_95_2: bool = ELUsingAArch32(state, tracer, s_95_1);
        // D s_95_3: write-var gs#126079 <= s_95_2
        fn_state.gs_126079 = s_95_2;
        // N s_95_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_96_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_96_1: call __IMPDEF_boolean(s_96_0)
        let s_96_1: bool = u__IMPDEF_boolean(state, tracer, s_96_0);
        // D s_96_2: write-var gs#126078 <= s_96_1
        fn_state.gs_126078 = s_96_1;
        // N s_96_3: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #() : ()
        let s_97_0: () = ();
        // S s_97_1: call EDSCR_read(s_97_0)
        let s_97_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_97_0);
        // S s_97_2: call _get_EDSCR_Type_SDD(s_97_1)
        let s_97_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_97_1);
        // S s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // C s_97_4: const #1u : u8
        let s_97_4: bool = true;
        // C s_97_5: cast zx s_97_4 -> bv
        let s_97_5: Bits = Bits::new(s_97_4 as u128, 1u16);
        // S s_97_6: cmp-eq s_97_3 s_97_5
        let s_97_6: bool = ((s_97_3) == (s_97_5));
        // D s_97_7: write-var gs#126077 <= s_97_6
        fn_state.gs_126077 = s_97_6;
        // N s_97_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #424u : u32
        let s_98_0: u32 = 424;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // C s_98_2: const #2u : u8
        let s_98_2: u8 = 2;
        // D s_98_3: cmp-lt s_98_1 s_98_2
        let s_98_3: bool = ((s_98_1) < (s_98_2));
        // D s_98_4: write-var gs#126076 <= s_98_3
        fn_state.gs_126076 = s_98_3;
        // N s_98_5: jump b27
        return block_27(state, tracer, fn_state);
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
        // D s_100_0: read-var __MDCR_EL3_TDCC:u8
        let s_100_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 1u16);
        // C s_100_2: const #1u : u8
        let s_100_2: bool = true;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: write-var gs#126075 <= s_100_4
        fn_state.gs_126075 = s_100_4;
        // N s_100_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #424u : u32
        let s_101_0: u32 = 424;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call ELUsingAArch32(s_101_1)
        let s_101_2: bool = ELUsingAArch32(state, tracer, s_101_1);
        // D s_101_3: not s_101_2
        let s_101_3: bool = !s_101_2;
        // D s_101_4: write-var gs#126074 <= s_101_3
        fn_state.gs_126074 = s_101_3;
        // N s_101_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_102_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_102_1: call __IMPDEF_boolean(s_102_0)
        let s_102_1: bool = u__IMPDEF_boolean(state, tracer, s_102_0);
        // D s_102_2: write-var gs#126073 <= s_102_1
        fn_state.gs_126073 = s_102_1;
        // N s_102_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call EDSCR_read(s_103_0)
        let s_103_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_103_0);
        // S s_103_2: call _get_EDSCR_Type_SDD(s_103_1)
        let s_103_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_103_1);
        // S s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // C s_103_4: const #1u : u8
        let s_103_4: bool = true;
        // C s_103_5: cast zx s_103_4 -> bv
        let s_103_5: Bits = Bits::new(s_103_4 as u128, 1u16);
        // S s_103_6: cmp-eq s_103_3 s_103_5
        let s_103_6: bool = ((s_103_3) == (s_103_5));
        // D s_103_7: write-var gs#126072 <= s_103_6
        fn_state.gs_126072 = s_103_6;
        // N s_103_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #424u : u32
        let s_104_0: u32 = 424;
        // D s_104_1: read-reg s_104_0:u8
        let s_104_1: u8 = {
            let value = state.read_register::<u8>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // C s_104_2: const #2u : u8
        let s_104_2: u8 = 2;
        // D s_104_3: cmp-lt s_104_1 s_104_2
        let s_104_3: bool = ((s_104_1) < (s_104_2));
        // D s_104_4: write-var gs#126071 <= s_104_3
        fn_state.gs_126071 = s_104_3;
        // N s_104_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call Halted(s_105_0)
        let s_105_1: bool = Halted(state, tracer, s_105_0);
        // N s_105_2: branch s_105_1 b227 b106
        if s_105_1 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#126095 <= s_106_0
        fn_state.gs_126095 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#126095:u8
        let s_107_0: bool = fn_state.gs_126095;
        // N s_107_1: branch s_107_0 b226 b108
        if s_107_0 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#126096 <= s_108_0
        fn_state.gs_126096 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#126096:u8
        let s_109_0: bool = fn_state.gs_126096;
        // N s_109_1: branch s_109_0 b225 b110
        if s_109_0 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#126097 <= s_110_0
        fn_state.gs_126097 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#126097:u8
        let s_111_0: bool = fn_state.gs_126097;
        // N s_111_1: branch s_111_0 b224 b112
        if s_111_0 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#126098 <= s_112_0
        fn_state.gs_126098 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#126098:u8
        let s_113_0: bool = fn_state.gs_126098;
        // N s_113_1: branch s_113_0 b223 b114
        if s_113_0 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#126099 <= s_114_0
        fn_state.gs_126099 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#126099:u8
        let s_115_0: bool = fn_state.gs_126099;
        // N s_115_1: branch s_115_0 b222 b116
        if s_115_0 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call Halted(s_116_0)
        let s_116_1: bool = Halted(state, tracer, s_116_0);
        // N s_116_2: branch s_116_1 b221 b117
        if s_116_1 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#126100 <= s_117_0
        fn_state.gs_126100 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#126100:u8
        let s_118_0: bool = fn_state.gs_126100;
        // N s_118_1: branch s_118_0 b220 b119
        if s_118_0 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#126101 <= s_119_0
        fn_state.gs_126101 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#126101:u8
        let s_120_0: bool = fn_state.gs_126101;
        // N s_120_1: branch s_120_0 b219 b121
        if s_120_0 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#126102 <= s_121_0
        fn_state.gs_126102 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#126102:u8
        let s_122_0: bool = fn_state.gs_126102;
        // N s_122_1: branch s_122_0 b218 b123
        if s_122_0 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#126103 <= s_123_0
        fn_state.gs_126103 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#126103:u8
        let s_124_0: bool = fn_state.gs_126103;
        // N s_124_1: branch s_124_0 b217 b125
        if s_124_0 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // D s_125_1: write-var gs#126104 <= s_125_0
        fn_state.gs_126104 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#126104:u8
        let s_126_0: bool = fn_state.gs_126104;
        // N s_126_1: branch s_126_0 b216 b127
        if s_126_0 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #() : ()
        let s_127_0: () = ();
        // S s_127_1: call Halted(s_127_0)
        let s_127_1: bool = Halted(state, tracer, s_127_0);
        // N s_127_2: branch s_127_1 b215 b128
        if s_127_1 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#126105 <= s_128_0
        fn_state.gs_126105 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#126105:u8
        let s_129_0: bool = fn_state.gs_126105;
        // N s_129_1: branch s_129_0 b214 b130
        if s_129_0 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#126106 <= s_130_0
        fn_state.gs_126106 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#126106:u8
        let s_131_0: bool = fn_state.gs_126106;
        // N s_131_1: branch s_131_0 b213 b132
        if s_131_0 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#126107 <= s_132_0
        fn_state.gs_126107 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#126107:u8
        let s_133_0: bool = fn_state.gs_126107;
        // N s_133_1: branch s_133_0 b212 b134
        if s_133_0 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#126108 <= s_134_0
        fn_state.gs_126108 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#126108:u8
        let s_135_0: bool = fn_state.gs_126108;
        // N s_135_1: branch s_135_0 b211 b136
        if s_135_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#126109 <= s_136_0
        fn_state.gs_126109 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#126109:u8
        let s_137_0: bool = fn_state.gs_126109;
        // N s_137_1: branch s_137_0 b210 b138
        if s_137_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #() : ()
        let s_138_0: () = ();
        // S s_138_1: call EL2Enabled(s_138_0)
        let s_138_1: bool = EL2Enabled(state, tracer, s_138_0);
        // N s_138_2: branch s_138_1 b209 b139
        if s_138_1 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#126110 <= s_139_0
        fn_state.gs_126110 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#126110:u8
        let s_140_0: bool = fn_state.gs_126110;
        // N s_140_1: branch s_140_0 b208 b141
        if s_140_0 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #0u : u8
        let s_141_0: bool = false;
        // D s_141_1: write-var gs#126111 <= s_141_0
        fn_state.gs_126111 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#126111:u8
        let s_142_0: bool = fn_state.gs_126111;
        // N s_142_1: branch s_142_0 b207 b143
        if s_142_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #() : ()
        let s_143_0: () = ();
        // S s_143_1: call EL2Enabled(s_143_0)
        let s_143_1: bool = EL2Enabled(state, tracer, s_143_0);
        // N s_143_2: branch s_143_1 b206 b144
        if s_143_1 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var gs#126112 <= s_144_0
        fn_state.gs_126112 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#126112:u8
        let s_145_0: bool = fn_state.gs_126112;
        // N s_145_1: branch s_145_0 b205 b146
        if s_145_0 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #0u : u8
        let s_146_0: bool = false;
        // D s_146_1: write-var gs#126113 <= s_146_0
        fn_state.gs_126113 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#126113:u8
        let s_147_0: bool = fn_state.gs_126113;
        // N s_147_1: branch s_147_0 b204 b148
        if s_147_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #() : ()
        let s_148_0: () = ();
        // S s_148_1: call EL2Enabled(s_148_0)
        let s_148_1: bool = EL2Enabled(state, tracer, s_148_0);
        // N s_148_2: branch s_148_1 b203 b149
        if s_148_1 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #0u : u8
        let s_149_0: bool = false;
        // D s_149_1: write-var gs#126114 <= s_149_0
        fn_state.gs_126114 = s_149_0;
        // N s_149_2: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var gs#126114:u8
        let s_150_0: bool = fn_state.gs_126114;
        // N s_150_1: branch s_150_0 b202 b151
        if s_150_0 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#126115 <= s_151_0
        fn_state.gs_126115 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#126115:u8
        let s_152_0: bool = fn_state.gs_126115;
        // N s_152_1: branch s_152_0 b201 b153
        if s_152_0 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #() : ()
        let s_153_0: () = ();
        // S s_153_1: call EL2Enabled(s_153_0)
        let s_153_1: bool = EL2Enabled(state, tracer, s_153_0);
        // N s_153_2: branch s_153_1 b200 b154
        if s_153_1 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#126116 <= s_154_0
        fn_state.gs_126116 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#126116:u8
        let s_155_0: bool = fn_state.gs_126116;
        // N s_155_1: branch s_155_0 b199 b156
        if s_155_0 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#126117 <= s_156_0
        fn_state.gs_126117 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#126117:u8
        let s_157_0: bool = fn_state.gs_126117;
        // N s_157_1: branch s_157_0 b198 b158
        if s_157_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #424u : u32
        let s_158_0: u32 = 424;
        // D s_158_1: read-reg s_158_0:u8
        let s_158_1: u8 = {
            let value = state.read_register::<u8>(s_158_0 as isize);
            tracer.read_register(s_158_0 as isize, value);
            value
        };
        // C s_158_2: const #2u : u8
        let s_158_2: u8 = 2;
        // D s_158_3: cmp-lt s_158_1 s_158_2
        let s_158_3: bool = ((s_158_1) < (s_158_2));
        // N s_158_4: branch s_158_3 b197 b159
        if s_158_3 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #0u : u8
        let s_159_0: bool = false;
        // D s_159_1: write-var gs#126118 <= s_159_0
        fn_state.gs_126118 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#126118:u8
        let s_160_0: bool = fn_state.gs_126118;
        // N s_160_1: branch s_160_0 b196 b161
        if s_160_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0u : u8
        let s_161_0: bool = false;
        // D s_161_1: write-var gs#126119 <= s_161_0
        fn_state.gs_126119 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#126119:u8
        let s_162_0: bool = fn_state.gs_126119;
        // N s_162_1: branch s_162_0 b190 b163
        if s_162_0 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #424u : u32
        let s_163_0: u32 = 424;
        // D s_163_1: read-reg s_163_0:u8
        let s_163_1: u8 = {
            let value = state.read_register::<u8>(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // C s_163_2: const #2u : u8
        let s_163_2: u8 = 2;
        // D s_163_3: cmp-lt s_163_1 s_163_2
        let s_163_3: bool = ((s_163_1) < (s_163_2));
        // N s_163_4: branch s_163_3 b189 b164
        if s_163_3 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #0u : u8
        let s_164_0: bool = false;
        // D s_164_1: write-var gs#126120 <= s_164_0
        fn_state.gs_126120 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#126120:u8
        let s_165_0: bool = fn_state.gs_126120;
        // N s_165_1: branch s_165_0 b188 b166
        if s_165_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #0u : u8
        let s_166_0: bool = false;
        // D s_166_1: write-var gs#126121 <= s_166_0
        fn_state.gs_126121 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#126121:u8
        let s_167_0: bool = fn_state.gs_126121;
        // N s_167_1: branch s_167_0 b182 b168
        if s_167_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #424u : u32
        let s_168_0: u32 = 424;
        // D s_168_1: read-reg s_168_0:u8
        let s_168_1: u8 = {
            let value = state.read_register::<u8>(s_168_0 as isize);
            tracer.read_register(s_168_0 as isize, value);
            value
        };
        // C s_168_2: const #2u : u8
        let s_168_2: u8 = 2;
        // D s_168_3: cmp-lt s_168_1 s_168_2
        let s_168_3: bool = ((s_168_1) < (s_168_2));
        // N s_168_4: branch s_168_3 b181 b169
        if s_168_3 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #0u : u8
        let s_169_0: bool = false;
        // D s_169_1: write-var gs#126122 <= s_169_0
        fn_state.gs_126122 = s_169_0;
        // N s_169_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#126122:u8
        let s_170_0: bool = fn_state.gs_126122;
        // N s_170_1: branch s_170_0 b180 b171
        if s_170_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #0u : u8
        let s_171_0: bool = false;
        // D s_171_1: write-var gs#126123 <= s_171_0
        fn_state.gs_126123 = s_171_0;
        // N s_171_2: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var gs#126123:u8
        let s_172_0: bool = fn_state.gs_126123;
        // N s_172_1: branch s_172_0 b174 b173
        if s_172_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var t:i
        let s_173_0: i128 = fn_state.t;
        // D s_173_1: call R_read(s_173_0)
        let s_173_1: u32 = R_read(state, tracer, s_173_0);
        // D s_173_2: call Mk_DBGDCCINT_Type(s_173_1)
        let s_173_2: ProductType700c18a878c5601b = Mk_DBGDCCINT_Type(
            state,
            tracer,
            s_173_1,
        );
        // D s_173_3: call DBGDCCINT_write(s_173_2)
        let s_173_3: () = DBGDCCINT_write(state, tracer, s_173_2);
        // N s_173_4: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #() : ()
        let s_174_0: () = ();
        // S s_174_1: call Halted(s_174_0)
        let s_174_1: bool = Halted(state, tracer, s_174_0);
        // N s_174_2: branch s_174_1 b179 b175
        if s_174_1 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_175(state, tracer, fn_state);
        };
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #0u : u8
        let s_175_0: bool = false;
        // D s_175_1: write-var gs#126124 <= s_175_0
        fn_state.gs_126124 = s_175_0;
        // N s_175_2: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var gs#126124:u8
        let s_176_0: bool = fn_state.gs_126124;
        // N s_176_1: branch s_176_0 b178 b177
        if s_176_0 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #5u : u8
        let s_177_0: u8 = 5;
        // C s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 8u16);
        // C s_177_2: cast zx s_177_1 -> i
        let s_177_2: i128 = (s_177_1.value() as i128);
        // C s_177_3: cast reint s_177_2 -> i64
        let s_177_3: i64 = (s_177_2 as i64);
        // C s_177_4: cast zx s_177_3 -> i
        let s_177_4: i128 = (i128::try_from(s_177_3).unwrap());
        // C s_177_5: const #424u : u32
        let s_177_5: u32 = 424;
        // D s_177_6: read-reg s_177_5:u8
        let s_177_6: u8 = {
            let value = state.read_register::<u8>(s_177_5 as isize);
            tracer.read_register(s_177_5 as isize, value);
            value
        };
        // D s_177_7: call AArch64_AArch32SystemAccessTrap(s_177_6, s_177_4)
        let s_177_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_177_6,
            s_177_4,
        );
        // N s_177_8: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_178_0: panic
        panic!("{:?}", ());
        // N s_178_1: return
        return;
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #() : ()
        let s_179_0: () = ();
        // S s_179_1: call EDSCR_read(s_179_0)
        let s_179_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_179_0);
        // S s_179_2: call _get_EDSCR_Type_SDD(s_179_1)
        let s_179_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_179_1);
        // S s_179_3: cast zx s_179_2 -> bv
        let s_179_3: Bits = Bits::new(s_179_2 as u128, 1u16);
        // C s_179_4: const #1u : u8
        let s_179_4: bool = true;
        // C s_179_5: cast zx s_179_4 -> bv
        let s_179_5: Bits = Bits::new(s_179_4 as u128, 1u16);
        // S s_179_6: cmp-eq s_179_3 s_179_5
        let s_179_6: bool = ((s_179_3) == (s_179_5));
        // D s_179_7: write-var gs#126124 <= s_179_6
        fn_state.gs_126124 = s_179_6;
        // N s_179_8: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var __MDCR_EL3_TDA:u8
        let s_180_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 1u16);
        // C s_180_2: const #1u : u8
        let s_180_2: bool = true;
        // C s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 1u16);
        // D s_180_4: cmp-eq s_180_1 s_180_3
        let s_180_4: bool = ((s_180_1) == (s_180_3));
        // D s_180_5: write-var gs#126123 <= s_180_4
        fn_state.gs_126123 = s_180_4;
        // N s_180_6: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #424u : u32
        let s_181_0: u32 = 424;
        // D s_181_1: read-reg s_181_0:u8
        let s_181_1: u8 = {
            let value = state.read_register::<u8>(s_181_0 as isize);
            tracer.read_register(s_181_0 as isize, value);
            value
        };
        // D s_181_2: call ELUsingAArch32(s_181_1)
        let s_181_2: bool = ELUsingAArch32(state, tracer, s_181_1);
        // D s_181_3: not s_181_2
        let s_181_3: bool = !s_181_2;
        // D s_181_4: write-var gs#126122 <= s_181_3
        fn_state.gs_126122 = s_181_3;
        // N s_181_5: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #() : ()
        let s_182_0: () = ();
        // S s_182_1: call Halted(s_182_0)
        let s_182_1: bool = Halted(state, tracer, s_182_0);
        // N s_182_2: branch s_182_1 b187 b183
        if s_182_1 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #0u : u8
        let s_183_0: bool = false;
        // D s_183_1: write-var gs#126125 <= s_183_0
        fn_state.gs_126125 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#126125:u8
        let s_184_0: bool = fn_state.gs_126125;
        // N s_184_1: branch s_184_0 b186 b185
        if s_184_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #() : ()
        let s_185_0: () = ();
        // S s_185_1: call AArch32_TakeMonitorTrapException(s_185_0)
        let s_185_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_185_0);
        // N s_185_2: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_186_0: panic
        panic!("{:?}", ());
        // N s_186_1: return
        return;
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #() : ()
        let s_187_0: () = ();
        // S s_187_1: call EDSCR_read(s_187_0)
        let s_187_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_187_0);
        // S s_187_2: call _get_EDSCR_Type_SDD(s_187_1)
        let s_187_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_187_1);
        // S s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 1u16);
        // C s_187_4: const #1u : u8
        let s_187_4: bool = true;
        // C s_187_5: cast zx s_187_4 -> bv
        let s_187_5: Bits = Bits::new(s_187_4 as u128, 1u16);
        // S s_187_6: cmp-eq s_187_3 s_187_5
        let s_187_6: bool = ((s_187_3) == (s_187_5));
        // D s_187_7: write-var gs#126125 <= s_187_6
        fn_state.gs_126125 = s_187_6;
        // N s_187_8: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var __SDCR_TDCC:u8
        let s_188_0: bool = fn_state.u__SDCR_TDCC;
        // D s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 1u16);
        // C s_188_2: const #1u : u8
        let s_188_2: bool = true;
        // C s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 1u16);
        // D s_188_4: cmp-eq s_188_1 s_188_3
        let s_188_4: bool = ((s_188_1) == (s_188_3));
        // D s_188_5: write-var gs#126121 <= s_188_4
        fn_state.gs_126121 = s_188_4;
        // N s_188_6: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #424u : u32
        let s_189_0: u32 = 424;
        // D s_189_1: read-reg s_189_0:u8
        let s_189_1: u8 = {
            let value = state.read_register::<u8>(s_189_0 as isize);
            tracer.read_register(s_189_0 as isize, value);
            value
        };
        // D s_189_2: call ELUsingAArch32(s_189_1)
        let s_189_2: bool = ELUsingAArch32(state, tracer, s_189_1);
        // D s_189_3: write-var gs#126120 <= s_189_2
        fn_state.gs_126120 = s_189_2;
        // N s_189_4: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #() : ()
        let s_190_0: () = ();
        // S s_190_1: call Halted(s_190_0)
        let s_190_1: bool = Halted(state, tracer, s_190_0);
        // N s_190_2: branch s_190_1 b195 b191
        if s_190_1 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #0u : u8
        let s_191_0: bool = false;
        // D s_191_1: write-var gs#126126 <= s_191_0
        fn_state.gs_126126 = s_191_0;
        // N s_191_2: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var gs#126126:u8
        let s_192_0: bool = fn_state.gs_126126;
        // N s_192_1: branch s_192_0 b194 b193
        if s_192_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #5u : u8
        let s_193_0: u8 = 5;
        // C s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 8u16);
        // C s_193_2: cast zx s_193_1 -> i
        let s_193_2: i128 = (s_193_1.value() as i128);
        // C s_193_3: cast reint s_193_2 -> i64
        let s_193_3: i64 = (s_193_2 as i64);
        // C s_193_4: cast zx s_193_3 -> i
        let s_193_4: i128 = (i128::try_from(s_193_3).unwrap());
        // C s_193_5: const #424u : u32
        let s_193_5: u32 = 424;
        // D s_193_6: read-reg s_193_5:u8
        let s_193_6: u8 = {
            let value = state.read_register::<u8>(s_193_5 as isize);
            tracer.read_register(s_193_5 as isize, value);
            value
        };
        // D s_193_7: call AArch64_AArch32SystemAccessTrap(s_193_6, s_193_4)
        let s_193_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_193_6,
            s_193_4,
        );
        // N s_193_8: return
        return;
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_194_0: panic
        panic!("{:?}", ());
        // N s_194_1: return
        return;
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #() : ()
        let s_195_0: () = ();
        // S s_195_1: call EDSCR_read(s_195_0)
        let s_195_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_195_0);
        // S s_195_2: call _get_EDSCR_Type_SDD(s_195_1)
        let s_195_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_195_1);
        // S s_195_3: cast zx s_195_2 -> bv
        let s_195_3: Bits = Bits::new(s_195_2 as u128, 1u16);
        // C s_195_4: const #1u : u8
        let s_195_4: bool = true;
        // C s_195_5: cast zx s_195_4 -> bv
        let s_195_5: Bits = Bits::new(s_195_4 as u128, 1u16);
        // S s_195_6: cmp-eq s_195_3 s_195_5
        let s_195_6: bool = ((s_195_3) == (s_195_5));
        // D s_195_7: write-var gs#126126 <= s_195_6
        fn_state.gs_126126 = s_195_6;
        // N s_195_8: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var __MDCR_EL3_TDCC:u8
        let s_196_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_196_1: cast zx s_196_0 -> bv
        let s_196_1: Bits = Bits::new(s_196_0 as u128, 1u16);
        // C s_196_2: const #1u : u8
        let s_196_2: bool = true;
        // C s_196_3: cast zx s_196_2 -> bv
        let s_196_3: Bits = Bits::new(s_196_2 as u128, 1u16);
        // D s_196_4: cmp-eq s_196_1 s_196_3
        let s_196_4: bool = ((s_196_1) == (s_196_3));
        // D s_196_5: write-var gs#126119 <= s_196_4
        fn_state.gs_126119 = s_196_4;
        // N s_196_6: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #424u : u32
        let s_197_0: u32 = 424;
        // D s_197_1: read-reg s_197_0:u8
        let s_197_1: u8 = {
            let value = state.read_register::<u8>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: call ELUsingAArch32(s_197_1)
        let s_197_2: bool = ELUsingAArch32(state, tracer, s_197_1);
        // D s_197_3: not s_197_2
        let s_197_3: bool = !s_197_2;
        // D s_197_4: write-var gs#126118 <= s_197_3
        fn_state.gs_126118 = s_197_3;
        // N s_197_5: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #5u : u8
        let s_198_0: u8 = 5;
        // C s_198_1: cast zx s_198_0 -> bv
        let s_198_1: Bits = Bits::new(s_198_0 as u128, 8u16);
        // C s_198_2: cast zx s_198_1 -> i
        let s_198_2: i128 = (s_198_1.value() as i128);
        // C s_198_3: cast reint s_198_2 -> i64
        let s_198_3: i64 = (s_198_2 as i64);
        // C s_198_4: cast zx s_198_3 -> i
        let s_198_4: i128 = (i128::try_from(s_198_3).unwrap());
        // S s_198_5: call AArch32_TakeHypTrapException(s_198_4)
        let s_198_5: () = AArch32_TakeHypTrapException(state, tracer, s_198_4);
        // N s_198_6: return
        return;
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #() : ()
        let s_199_0: () = ();
        // S s_199_1: call HDCR_read(s_199_0)
        let s_199_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_199_0);
        // S s_199_2: call _get_HDCR_Type_TDE(s_199_1)
        let s_199_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_199_1);
        // C s_199_3: const #() : ()
        let s_199_3: () = ();
        // S s_199_4: call HDCR_read(s_199_3)
        let s_199_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_199_3);
        // S s_199_5: call _get_HDCR_Type_TDA(s_199_4)
        let s_199_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_199_4);
        // S s_199_6: cast zx s_199_2 -> bv
        let s_199_6: Bits = Bits::new(s_199_2 as u128, 1u16);
        // S s_199_7: cast zx s_199_5 -> bv
        let s_199_7: Bits = Bits::new(s_199_5 as u128, 1u16);
        // S s_199_8: cast reint s_199_6 -> u128
        let s_199_8: u128 = (s_199_6.value() as u128);
        // D s_199_9: size-of s_199_6
        let s_199_9: u16 = s_199_6.length();
        // S s_199_10: cast reint s_199_7 -> u128
        let s_199_10: u128 = (s_199_7.value() as u128);
        // D s_199_11: size-of s_199_7
        let s_199_11: u16 = s_199_7.length();
        // D s_199_12: lsl s_199_8 s_199_11
        let s_199_12: u128 = s_199_8 << s_199_11;
        // D s_199_13: or s_199_12 s_199_10
        let s_199_13: u128 = ((s_199_12) | (s_199_10));
        // D s_199_14: add s_199_9 s_199_11
        let s_199_14: u16 = (s_199_9 + s_199_11);
        // D s_199_15: create-bits s_199_13 s_199_14
        let s_199_15: Bits = Bits::new(s_199_13, s_199_14);
        // D s_199_16: cast reint s_199_15 -> u8
        let s_199_16: u8 = (s_199_15.value() as u8);
        // D s_199_17: cast zx s_199_16 -> bv
        let s_199_17: Bits = Bits::new(s_199_16 as u128, 2u16);
        // C s_199_18: const #0u : u8
        let s_199_18: u8 = 0;
        // C s_199_19: cast zx s_199_18 -> bv
        let s_199_19: Bits = Bits::new(s_199_18 as u128, 2u16);
        // D s_199_20: cmp-ne s_199_17 s_199_19
        let s_199_20: bool = ((s_199_17) != (s_199_19));
        // D s_199_21: write-var gs#126117 <= s_199_20
        fn_state.gs_126117 = s_199_20;
        // N s_199_22: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #432u : u32
        let s_200_0: u32 = 432;
        // D s_200_1: read-reg s_200_0:u8
        let s_200_1: u8 = {
            let value = state.read_register::<u8>(s_200_0 as isize);
            tracer.read_register(s_200_0 as isize, value);
            value
        };
        // D s_200_2: call ELUsingAArch32(s_200_1)
        let s_200_2: bool = ELUsingAArch32(state, tracer, s_200_1);
        // D s_200_3: write-var gs#126116 <= s_200_2
        fn_state.gs_126116 = s_200_2;
        // N s_200_4: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #5u : u8
        let s_201_0: u8 = 5;
        // C s_201_1: cast zx s_201_0 -> bv
        let s_201_1: Bits = Bits::new(s_201_0 as u128, 8u16);
        // C s_201_2: cast zx s_201_1 -> i
        let s_201_2: i128 = (s_201_1.value() as i128);
        // C s_201_3: cast reint s_201_2 -> i64
        let s_201_3: i64 = (s_201_2 as i64);
        // C s_201_4: cast zx s_201_3 -> i
        let s_201_4: i128 = (i128::try_from(s_201_3).unwrap());
        // C s_201_5: const #432u : u32
        let s_201_5: u32 = 432;
        // D s_201_6: read-reg s_201_5:u8
        let s_201_6: u8 = {
            let value = state.read_register::<u8>(s_201_5 as isize);
            tracer.read_register(s_201_5 as isize, value);
            value
        };
        // D s_201_7: call AArch64_AArch32SystemAccessTrap(s_201_6, s_201_4)
        let s_201_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_201_6,
            s_201_4,
        );
        // N s_201_8: return
        return;
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #104880u : u32
        let s_202_0: u32 = 104880;
        // D s_202_1: read-reg s_202_0:struct
        let s_202_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_202_0 as isize);
            tracer.read_register(s_202_0 as isize, value);
            value
        };
        // D s_202_2: call _get_MDCR_EL2_Type_TDE(s_202_1)
        let s_202_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_202_1);
        // C s_202_3: const #104880u : u32
        let s_202_3: u32 = 104880;
        // D s_202_4: read-reg s_202_3:struct
        let s_202_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_202_3 as isize);
            tracer.read_register(s_202_3 as isize, value);
            value
        };
        // D s_202_5: call _get_MDCR_EL2_Type_TDA(s_202_4)
        let s_202_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_202_4);
        // D s_202_6: cast zx s_202_2 -> bv
        let s_202_6: Bits = Bits::new(s_202_2 as u128, 1u16);
        // D s_202_7: cast zx s_202_5 -> bv
        let s_202_7: Bits = Bits::new(s_202_5 as u128, 1u16);
        // D s_202_8: cast reint s_202_6 -> u128
        let s_202_8: u128 = (s_202_6.value() as u128);
        // D s_202_9: size-of s_202_6
        let s_202_9: u16 = s_202_6.length();
        // D s_202_10: cast reint s_202_7 -> u128
        let s_202_10: u128 = (s_202_7.value() as u128);
        // D s_202_11: size-of s_202_7
        let s_202_11: u16 = s_202_7.length();
        // D s_202_12: lsl s_202_8 s_202_11
        let s_202_12: u128 = s_202_8 << s_202_11;
        // D s_202_13: or s_202_12 s_202_10
        let s_202_13: u128 = ((s_202_12) | (s_202_10));
        // D s_202_14: add s_202_9 s_202_11
        let s_202_14: u16 = (s_202_9 + s_202_11);
        // D s_202_15: create-bits s_202_13 s_202_14
        let s_202_15: Bits = Bits::new(s_202_13, s_202_14);
        // D s_202_16: cast reint s_202_15 -> u8
        let s_202_16: u8 = (s_202_15.value() as u8);
        // D s_202_17: cast zx s_202_16 -> bv
        let s_202_17: Bits = Bits::new(s_202_16 as u128, 2u16);
        // C s_202_18: const #0u : u8
        let s_202_18: u8 = 0;
        // C s_202_19: cast zx s_202_18 -> bv
        let s_202_19: Bits = Bits::new(s_202_18 as u128, 2u16);
        // D s_202_20: cmp-ne s_202_17 s_202_19
        let s_202_20: bool = ((s_202_17) != (s_202_19));
        // D s_202_21: write-var gs#126115 <= s_202_20
        fn_state.gs_126115 = s_202_20;
        // N s_202_22: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #432u : u32
        let s_203_0: u32 = 432;
        // D s_203_1: read-reg s_203_0:u8
        let s_203_1: u8 = {
            let value = state.read_register::<u8>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call ELUsingAArch32(s_203_1)
        let s_203_2: bool = ELUsingAArch32(state, tracer, s_203_1);
        // D s_203_3: not s_203_2
        let s_203_3: bool = !s_203_2;
        // D s_203_4: write-var gs#126114 <= s_203_3
        fn_state.gs_126114 = s_203_3;
        // N s_203_5: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #5u : u8
        let s_204_0: u8 = 5;
        // C s_204_1: cast zx s_204_0 -> bv
        let s_204_1: Bits = Bits::new(s_204_0 as u128, 8u16);
        // C s_204_2: cast zx s_204_1 -> i
        let s_204_2: i128 = (s_204_1.value() as i128);
        // C s_204_3: cast reint s_204_2 -> i64
        let s_204_3: i64 = (s_204_2 as i64);
        // C s_204_4: cast zx s_204_3 -> i
        let s_204_4: i128 = (i128::try_from(s_204_3).unwrap());
        // S s_204_5: call AArch32_TakeHypTrapException(s_204_4)
        let s_204_5: () = AArch32_TakeHypTrapException(state, tracer, s_204_4);
        // N s_204_6: return
        return;
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var __HDCR_TDCC:u8
        let s_205_0: bool = fn_state.u__HDCR_TDCC;
        // D s_205_1: cast zx s_205_0 -> bv
        let s_205_1: Bits = Bits::new(s_205_0 as u128, 1u16);
        // C s_205_2: const #1u : u8
        let s_205_2: bool = true;
        // C s_205_3: cast zx s_205_2 -> bv
        let s_205_3: Bits = Bits::new(s_205_2 as u128, 1u16);
        // D s_205_4: cmp-eq s_205_1 s_205_3
        let s_205_4: bool = ((s_205_1) == (s_205_3));
        // D s_205_5: write-var gs#126113 <= s_205_4
        fn_state.gs_126113 = s_205_4;
        // N s_205_6: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #432u : u32
        let s_206_0: u32 = 432;
        // D s_206_1: read-reg s_206_0:u8
        let s_206_1: u8 = {
            let value = state.read_register::<u8>(s_206_0 as isize);
            tracer.read_register(s_206_0 as isize, value);
            value
        };
        // D s_206_2: call ELUsingAArch32(s_206_1)
        let s_206_2: bool = ELUsingAArch32(state, tracer, s_206_1);
        // D s_206_3: write-var gs#126112 <= s_206_2
        fn_state.gs_126112 = s_206_2;
        // N s_206_4: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #5u : u8
        let s_207_0: u8 = 5;
        // C s_207_1: cast zx s_207_0 -> bv
        let s_207_1: Bits = Bits::new(s_207_0 as u128, 8u16);
        // C s_207_2: cast zx s_207_1 -> i
        let s_207_2: i128 = (s_207_1.value() as i128);
        // C s_207_3: cast reint s_207_2 -> i64
        let s_207_3: i64 = (s_207_2 as i64);
        // C s_207_4: cast zx s_207_3 -> i
        let s_207_4: i128 = (i128::try_from(s_207_3).unwrap());
        // C s_207_5: const #432u : u32
        let s_207_5: u32 = 432;
        // D s_207_6: read-reg s_207_5:u8
        let s_207_6: u8 = {
            let value = state.read_register::<u8>(s_207_5 as isize);
            tracer.read_register(s_207_5 as isize, value);
            value
        };
        // D s_207_7: call AArch64_AArch32SystemAccessTrap(s_207_6, s_207_4)
        let s_207_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_207_6,
            s_207_4,
        );
        // N s_207_8: return
        return;
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var __MDCR_EL2_TDCC:u8
        let s_208_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_208_1: cast zx s_208_0 -> bv
        let s_208_1: Bits = Bits::new(s_208_0 as u128, 1u16);
        // C s_208_2: const #1u : u8
        let s_208_2: bool = true;
        // C s_208_3: cast zx s_208_2 -> bv
        let s_208_3: Bits = Bits::new(s_208_2 as u128, 1u16);
        // D s_208_4: cmp-eq s_208_1 s_208_3
        let s_208_4: bool = ((s_208_1) == (s_208_3));
        // D s_208_5: write-var gs#126111 <= s_208_4
        fn_state.gs_126111 = s_208_4;
        // N s_208_6: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #432u : u32
        let s_209_0: u32 = 432;
        // D s_209_1: read-reg s_209_0:u8
        let s_209_1: u8 = {
            let value = state.read_register::<u8>(s_209_0 as isize);
            tracer.read_register(s_209_0 as isize, value);
            value
        };
        // D s_209_2: call ELUsingAArch32(s_209_1)
        let s_209_2: bool = ELUsingAArch32(state, tracer, s_209_1);
        // D s_209_3: not s_209_2
        let s_209_3: bool = !s_209_2;
        // D s_209_4: write-var gs#126110 <= s_209_3
        fn_state.gs_126110 = s_209_3;
        // N s_209_5: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_210_0: panic
        panic!("{:?}", ());
        // N s_210_1: return
        return;
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var __MDCR_EL3_TDA:u8
        let s_211_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_211_1: cast zx s_211_0 -> bv
        let s_211_1: Bits = Bits::new(s_211_0 as u128, 1u16);
        // C s_211_2: const #1u : u8
        let s_211_2: bool = true;
        // C s_211_3: cast zx s_211_2 -> bv
        let s_211_3: Bits = Bits::new(s_211_2 as u128, 1u16);
        // D s_211_4: cmp-eq s_211_1 s_211_3
        let s_211_4: bool = ((s_211_1) == (s_211_3));
        // D s_211_5: write-var gs#126109 <= s_211_4
        fn_state.gs_126109 = s_211_4;
        // N s_211_6: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #424u : u32
        let s_212_0: u32 = 424;
        // D s_212_1: read-reg s_212_0:u8
        let s_212_1: u8 = {
            let value = state.read_register::<u8>(s_212_0 as isize);
            tracer.read_register(s_212_0 as isize, value);
            value
        };
        // D s_212_2: call ELUsingAArch32(s_212_1)
        let s_212_2: bool = ELUsingAArch32(state, tracer, s_212_1);
        // D s_212_3: not s_212_2
        let s_212_3: bool = !s_212_2;
        // D s_212_4: write-var gs#126108 <= s_212_3
        fn_state.gs_126108 = s_212_3;
        // N s_212_5: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_213_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_213_1: call __IMPDEF_boolean(s_213_0)
        let s_213_1: bool = u__IMPDEF_boolean(state, tracer, s_213_0);
        // D s_213_2: write-var gs#126107 <= s_213_1
        fn_state.gs_126107 = s_213_1;
        // N s_213_3: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #() : ()
        let s_214_0: () = ();
        // S s_214_1: call EDSCR_read(s_214_0)
        let s_214_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_214_0);
        // S s_214_2: call _get_EDSCR_Type_SDD(s_214_1)
        let s_214_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_214_1);
        // S s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 1u16);
        // C s_214_4: const #1u : u8
        let s_214_4: bool = true;
        // C s_214_5: cast zx s_214_4 -> bv
        let s_214_5: Bits = Bits::new(s_214_4 as u128, 1u16);
        // S s_214_6: cmp-eq s_214_3 s_214_5
        let s_214_6: bool = ((s_214_3) == (s_214_5));
        // D s_214_7: write-var gs#126106 <= s_214_6
        fn_state.gs_126106 = s_214_6;
        // N s_214_8: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #424u : u32
        let s_215_0: u32 = 424;
        // D s_215_1: read-reg s_215_0:u8
        let s_215_1: u8 = {
            let value = state.read_register::<u8>(s_215_0 as isize);
            tracer.read_register(s_215_0 as isize, value);
            value
        };
        // C s_215_2: const #2u : u8
        let s_215_2: u8 = 2;
        // D s_215_3: cmp-lt s_215_1 s_215_2
        let s_215_3: bool = ((s_215_1) < (s_215_2));
        // D s_215_4: write-var gs#126105 <= s_215_3
        fn_state.gs_126105 = s_215_3;
        // N s_215_5: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_216_0: panic
        panic!("{:?}", ());
        // N s_216_1: return
        return;
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_217_0: read-var __SDCR_TDCC:u8
        let s_217_0: bool = fn_state.u__SDCR_TDCC;
        // D s_217_1: cast zx s_217_0 -> bv
        let s_217_1: Bits = Bits::new(s_217_0 as u128, 1u16);
        // C s_217_2: const #1u : u8
        let s_217_2: bool = true;
        // C s_217_3: cast zx s_217_2 -> bv
        let s_217_3: Bits = Bits::new(s_217_2 as u128, 1u16);
        // D s_217_4: cmp-eq s_217_1 s_217_3
        let s_217_4: bool = ((s_217_1) == (s_217_3));
        // D s_217_5: write-var gs#126104 <= s_217_4
        fn_state.gs_126104 = s_217_4;
        // N s_217_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #424u : u32
        let s_218_0: u32 = 424;
        // D s_218_1: read-reg s_218_0:u8
        let s_218_1: u8 = {
            let value = state.read_register::<u8>(s_218_0 as isize);
            tracer.read_register(s_218_0 as isize, value);
            value
        };
        // D s_218_2: call ELUsingAArch32(s_218_1)
        let s_218_2: bool = ELUsingAArch32(state, tracer, s_218_1);
        // D s_218_3: write-var gs#126103 <= s_218_2
        fn_state.gs_126103 = s_218_2;
        // N s_218_4: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_219_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_219_1: call __IMPDEF_boolean(s_219_0)
        let s_219_1: bool = u__IMPDEF_boolean(state, tracer, s_219_0);
        // D s_219_2: write-var gs#126102 <= s_219_1
        fn_state.gs_126102 = s_219_1;
        // N s_219_3: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #() : ()
        let s_220_0: () = ();
        // S s_220_1: call EDSCR_read(s_220_0)
        let s_220_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_220_0);
        // S s_220_2: call _get_EDSCR_Type_SDD(s_220_1)
        let s_220_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_220_1);
        // S s_220_3: cast zx s_220_2 -> bv
        let s_220_3: Bits = Bits::new(s_220_2 as u128, 1u16);
        // C s_220_4: const #1u : u8
        let s_220_4: bool = true;
        // C s_220_5: cast zx s_220_4 -> bv
        let s_220_5: Bits = Bits::new(s_220_4 as u128, 1u16);
        // S s_220_6: cmp-eq s_220_3 s_220_5
        let s_220_6: bool = ((s_220_3) == (s_220_5));
        // D s_220_7: write-var gs#126101 <= s_220_6
        fn_state.gs_126101 = s_220_6;
        // N s_220_8: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #424u : u32
        let s_221_0: u32 = 424;
        // D s_221_1: read-reg s_221_0:u8
        let s_221_1: u8 = {
            let value = state.read_register::<u8>(s_221_0 as isize);
            tracer.read_register(s_221_0 as isize, value);
            value
        };
        // C s_221_2: const #2u : u8
        let s_221_2: u8 = 2;
        // D s_221_3: cmp-lt s_221_1 s_221_2
        let s_221_3: bool = ((s_221_1) < (s_221_2));
        // D s_221_4: write-var gs#126100 <= s_221_3
        fn_state.gs_126100 = s_221_3;
        // N s_221_5: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_222_0: panic
        panic!("{:?}", ());
        // N s_222_1: return
        return;
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var __MDCR_EL3_TDCC:u8
        let s_223_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_223_1: cast zx s_223_0 -> bv
        let s_223_1: Bits = Bits::new(s_223_0 as u128, 1u16);
        // C s_223_2: const #1u : u8
        let s_223_2: bool = true;
        // C s_223_3: cast zx s_223_2 -> bv
        let s_223_3: Bits = Bits::new(s_223_2 as u128, 1u16);
        // D s_223_4: cmp-eq s_223_1 s_223_3
        let s_223_4: bool = ((s_223_1) == (s_223_3));
        // D s_223_5: write-var gs#126099 <= s_223_4
        fn_state.gs_126099 = s_223_4;
        // N s_223_6: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #424u : u32
        let s_224_0: u32 = 424;
        // D s_224_1: read-reg s_224_0:u8
        let s_224_1: u8 = {
            let value = state.read_register::<u8>(s_224_0 as isize);
            tracer.read_register(s_224_0 as isize, value);
            value
        };
        // D s_224_2: call ELUsingAArch32(s_224_1)
        let s_224_2: bool = ELUsingAArch32(state, tracer, s_224_1);
        // D s_224_3: not s_224_2
        let s_224_3: bool = !s_224_2;
        // D s_224_4: write-var gs#126098 <= s_224_3
        fn_state.gs_126098 = s_224_3;
        // N s_224_5: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_225_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_225_1: call __IMPDEF_boolean(s_225_0)
        let s_225_1: bool = u__IMPDEF_boolean(state, tracer, s_225_0);
        // D s_225_2: write-var gs#126097 <= s_225_1
        fn_state.gs_126097 = s_225_1;
        // N s_225_3: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #() : ()
        let s_226_0: () = ();
        // S s_226_1: call EDSCR_read(s_226_0)
        let s_226_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_226_0);
        // S s_226_2: call _get_EDSCR_Type_SDD(s_226_1)
        let s_226_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_226_1);
        // S s_226_3: cast zx s_226_2 -> bv
        let s_226_3: Bits = Bits::new(s_226_2 as u128, 1u16);
        // C s_226_4: const #1u : u8
        let s_226_4: bool = true;
        // C s_226_5: cast zx s_226_4 -> bv
        let s_226_5: Bits = Bits::new(s_226_4 as u128, 1u16);
        // S s_226_6: cmp-eq s_226_3 s_226_5
        let s_226_6: bool = ((s_226_3) == (s_226_5));
        // D s_226_7: write-var gs#126096 <= s_226_6
        fn_state.gs_126096 = s_226_6;
        // N s_226_8: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #424u : u32
        let s_227_0: u32 = 424;
        // D s_227_1: read-reg s_227_0:u8
        let s_227_1: u8 = {
            let value = state.read_register::<u8>(s_227_0 as isize);
            tracer.read_register(s_227_0 as isize, value);
            value
        };
        // C s_227_2: const #2u : u8
        let s_227_2: u8 = 2;
        // D s_227_3: cmp-lt s_227_1 s_227_2
        let s_227_3: bool = ((s_227_1) < (s_227_2));
        // D s_227_4: write-var gs#126095 <= s_227_3
        fn_state.gs_126095 = s_227_3;
        // N s_227_5: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_228_0: read-var t:i
        let s_228_0: i128 = fn_state.t;
        // D s_228_1: call R_read(s_228_0)
        let s_228_1: u32 = R_read(state, tracer, s_228_0);
        // D s_228_2: call Mk_DBGDCCINT_Type(s_228_1)
        let s_228_2: ProductType700c18a878c5601b = Mk_DBGDCCINT_Type(
            state,
            tracer,
            s_228_1,
        );
        // D s_228_3: call DBGDCCINT_write(s_228_2)
        let s_228_3: () = DBGDCCINT_write(state, tracer, s_228_2);
        // N s_228_4: return
        return;
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #70u : u32
        let s_229_0: u32 = 70;
        // S s_229_1: call ConstrainUnpredictableBool(s_229_0)
        let s_229_1: bool = ConstrainUnpredictableBool(state, tracer, s_229_0);
        // D s_229_2: write-var gs#126069 <= s_229_1
        fn_state.gs_126069 = s_229_1;
        // N s_229_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_230_0: panic
        panic!("{:?}", ());
        // N s_230_1: return
        return;
    }
}
