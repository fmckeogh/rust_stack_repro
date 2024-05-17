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
use DBGDSCRext_read::*;
use u_get_MDCR_EL2_Type_TDRA::*;
use Halted::*;
use DBGDRAR_read::*;
use HDCR_read::*;
use u_get_HDCR_Type_TDE::*;
use HCR_read::*;
use u__IMPDEF_boolean::*;
use u_get_HDCR_Type_TDRA::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_MDCR_EL2_Type_TDE::*;
use u_get_HCR_Type_TGE::*;
use ConstrainUnpredictableBool::*;
use u_get_MDCR_EL3_Type_TDA::*;
use u_get_DBGDSCRext_Type_UDCCdis::*;
use R_set::*;
use ELUsingAArch32::*;
use u__get_DBGDRAR::*;
use u_get_MDSCR_EL1_Type_TDCC::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn DBGDRAR_SysRegRead32_421a9884bf20a229<T: Tracer>(
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
        u__DBGDSCRext_UDCCdis: bool,
        gs_108920: bool,
        gs_108889: bool,
        gs_108916: bool,
        gs_108912: bool,
        gs_108887: bool,
        gs_108926: bool,
        ga_172327: ProductType5c790c8ef59cc8b2,
        gs_108896: bool,
        gs_108892: bool,
        gs_108888: bool,
        gs_108928: bool,
        gs_108929: bool,
        gs_108885: bool,
        gs_108917: bool,
        gs_108913: bool,
        gs_108894: bool,
        gs_108921: bool,
        u__PSTATE_EL: u8,
        gs_108898: bool,
        gs_108927: bool,
        gs_108884: bool,
        ga_172352: ProductType5c790c8ef59cc8b2,
        gs_108914: bool,
        ga_172285: ProductType5c790c8ef59cc8b2,
        ga_172213: ProductType5c790c8ef59cc8b2,
        gs_108901: bool,
        gs_108895: bool,
        gs_108883: bool,
        gs_108880: bool,
        gs_108908: bool,
        gs_108902: bool,
        ga_172357: ProductType5c790c8ef59cc8b2,
        gs_108909: bool,
        gs_108930: bool,
        u__MDCR_EL3_TDA: bool,
        gs_108907: bool,
        gs_108915: bool,
        gs_108919: bool,
        gs_108886: bool,
        gs_108918: bool,
        gs_108900: bool,
        gs_108903: bool,
        gs_108911: bool,
        gs_108893: bool,
        u__MDSCR_EL1_TDCC: bool,
        gs_108897: bool,
        gs_108924: bool,
        gs_108899: bool,
        gs_108925: bool,
        gs_108906: bool,
        gs_108910: bool,
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
        // C s_0_7: const #104648u : u32
        let s_0_7: u32 = 104648;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDSCR_EL1_Type_TDCC(s_0_8)
        let s_0_9: bool = u_get_MDSCR_EL1_Type_TDCC(state, tracer, s_0_8);
        // D s_0_10: write-var __MDSCR_EL1_TDCC <= s_0_9
        fn_state.u__MDSCR_EL1_TDCC = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call DBGDSCRext_read(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_0_11);
        // S s_0_13: call _get_DBGDSCRext_Type_UDCCdis(s_0_12)
        let s_0_13: bool = u_get_DBGDSCRext_Type_UDCCdis(state, tracer, s_0_12);
        // D s_0_14: write-var __DBGDSCRext_UDCCdis <= s_0_13
        fn_state.u__DBGDSCRext_UDCCdis = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call Halted(s_0_15)
        let s_0_16: bool = Halted(state, tracer, s_0_15);
        // N s_0_17: branch s_0_16 b175 b1
        if s_0_16 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#108880 <= s_1_0
        fn_state.gs_108880 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#108880:u8
        let s_2_0: bool = fn_state.gs_108880;
        // N s_2_1: branch s_2_0 b174 b3
        if s_2_0 {
            return block_174(state, tracer, fn_state);
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
        // C s_3_2: const #448u : u32
        let s_3_2: u32 = 448;
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
        // N s_3_6: branch s_3_5 b87 b4
        if s_3_5 {
            return block_87(state, tracer, fn_state);
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
        // N s_4_6: branch s_4_5 b40 b5
        if s_4_5 {
            return block_40(state, tracer, fn_state);
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
        // N s_5_6: branch s_5_5 b9 b6
        if s_5_5 {
            return block_9(state, tracer, fn_state);
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
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call DBGDRAR_read(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = DBGDRAR_read(state, tracer, s_8_0);
        // S s_8_2: call __get_DBGDRAR(s_8_1)
        let s_8_2: ProductType5c790c8ef59cc8b2 = u__get_DBGDRAR(state, tracer, s_8_1);
        // D s_8_3: write-var ga#172357 <= s_8_2
        fn_state.ga_172357 = s_8_2;
        // D s_8_4: read-var ga#172357.0:struct
        let s_8_4: u64 = fn_state.ga_172357._0;
        // C s_8_5: const #0s : i
        let s_8_5: i128 = 0;
        // D s_8_6: cast zx s_8_4 -> bv
        let s_8_6: Bits = Bits::new(s_8_4 as u128, 64u16);
        // C s_8_7: const #1s : i64
        let s_8_7: i64 = 1;
        // C s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // C s_8_9: const #31s : i
        let s_8_9: i128 = 31;
        // C s_8_10: add s_8_9 s_8_8
        let s_8_10: i128 = (s_8_9 + s_8_8);
        // D s_8_11: bit-extract s_8_6 s_8_5 s_8_10
        let s_8_11: Bits = (Bits::new(
            ((s_8_6) >> (s_8_5)).value(),
            u16::try_from(s_8_10).unwrap(),
        ));
        // D s_8_12: cast reint s_8_11 -> u32
        let s_8_12: u32 = (s_8_11.value() as u32);
        // D s_8_13: read-var t:i
        let s_8_13: i128 = fn_state.t;
        // D s_8_14: call R_set(s_8_13, s_8_12)
        let s_8_14: () = R_set(state, tracer, s_8_13, s_8_12);
        // N s_8_15: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call Halted(s_9_0)
        let s_9_1: bool = Halted(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b39 b10
        if s_9_1 {
            return block_39(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#108883 <= s_10_0
        fn_state.gs_108883 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#108883:u8
        let s_11_0: bool = fn_state.gs_108883;
        // N s_11_1: branch s_11_0 b38 b12
        if s_11_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#108884 <= s_12_0
        fn_state.gs_108884 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#108884:u8
        let s_13_0: bool = fn_state.gs_108884;
        // N s_13_1: branch s_13_0 b37 b14
        if s_13_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#108885 <= s_14_0
        fn_state.gs_108885 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#108885:u8
        let s_15_0: bool = fn_state.gs_108885;
        // N s_15_1: branch s_15_0 b36 b16
        if s_15_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#108886 <= s_16_0
        fn_state.gs_108886 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#108886:u8
        let s_17_0: bool = fn_state.gs_108886;
        // N s_17_1: branch s_17_0 b35 b18
        if s_17_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#108887 <= s_18_0
        fn_state.gs_108887 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#108887:u8
        let s_19_0: bool = fn_state.gs_108887;
        // N s_19_1: branch s_19_0 b34 b20
        if s_19_0 {
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
        // C s_20_0: const #424u : u32
        let s_20_0: u32 = 424;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // D s_20_3: cmp-lt s_20_1 s_20_2
        let s_20_3: bool = ((s_20_1) < (s_20_2));
        // N s_20_4: branch s_20_3 b33 b21
        if s_20_3 {
            return block_33(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#108888 <= s_21_0
        fn_state.gs_108888 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#108888:u8
        let s_22_0: bool = fn_state.gs_108888;
        // N s_22_1: branch s_22_0 b32 b23
        if s_22_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#108889 <= s_23_0
        fn_state.gs_108889 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#108889:u8
        let s_24_0: bool = fn_state.gs_108889;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
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
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call DBGDRAR_read(s_25_0)
        let s_25_1: ProductType5c790c8ef59cc8b2 = DBGDRAR_read(state, tracer, s_25_0);
        // S s_25_2: call __get_DBGDRAR(s_25_1)
        let s_25_2: ProductType5c790c8ef59cc8b2 = u__get_DBGDRAR(state, tracer, s_25_1);
        // D s_25_3: write-var ga#172352 <= s_25_2
        fn_state.ga_172352 = s_25_2;
        // D s_25_4: read-var ga#172352.0:struct
        let s_25_4: u64 = fn_state.ga_172352._0;
        // C s_25_5: const #0s : i
        let s_25_5: i128 = 0;
        // D s_25_6: cast zx s_25_4 -> bv
        let s_25_6: Bits = Bits::new(s_25_4 as u128, 64u16);
        // C s_25_7: const #1s : i64
        let s_25_7: i64 = 1;
        // C s_25_8: cast zx s_25_7 -> i
        let s_25_8: i128 = (i128::try_from(s_25_7).unwrap());
        // C s_25_9: const #31s : i
        let s_25_9: i128 = 31;
        // C s_25_10: add s_25_9 s_25_8
        let s_25_10: i128 = (s_25_9 + s_25_8);
        // D s_25_11: bit-extract s_25_6 s_25_5 s_25_10
        let s_25_11: Bits = (Bits::new(
            ((s_25_6) >> (s_25_5)).value(),
            u16::try_from(s_25_10).unwrap(),
        ));
        // D s_25_12: cast reint s_25_11 -> u32
        let s_25_12: u32 = (s_25_11.value() as u32);
        // D s_25_13: read-var t:i
        let s_25_13: i128 = fn_state.t;
        // D s_25_14: call R_set(s_25_13, s_25_12)
        let s_25_14: () = R_set(state, tracer, s_25_13, s_25_12);
        // N s_25_15: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call Halted(s_26_0)
        let s_26_1: bool = Halted(state, tracer, s_26_0);
        // N s_26_2: branch s_26_1 b31 b27
        if s_26_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#108892 <= s_27_0
        fn_state.gs_108892 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#108892:u8
        let s_28_0: bool = fn_state.gs_108892;
        // N s_28_1: branch s_28_0 b30 b29
        if s_28_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #5u : u8
        let s_29_0: u8 = 5;
        // C s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 8u16);
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (s_29_1.value() as i128);
        // C s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #424u : u32
        let s_29_5: u32 = 424;
        // D s_29_6: read-reg s_29_5:u8
        let s_29_6: u8 = {
            let value = state.read_register::<u8>(s_29_5 as isize);
            tracer.read_register(s_29_5 as isize, value);
            value
        };
        // D s_29_7: call AArch64_AArch32SystemAccessTrap(s_29_6, s_29_4)
        let s_29_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_29_6, s_29_4);
        // N s_29_8: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call EDSCR_read(s_31_0)
        let s_31_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_31_0);
        // S s_31_2: call _get_EDSCR_Type_SDD(s_31_1)
        let s_31_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_31_1);
        // S s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // C s_31_4: const #1u : u8
        let s_31_4: bool = true;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 1u16);
        // S s_31_6: cmp-eq s_31_3 s_31_5
        let s_31_6: bool = ((s_31_3) == (s_31_5));
        // D s_31_7: write-var gs#108892 <= s_31_6
        fn_state.gs_108892 = s_31_6;
        // N s_31_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MDCR_EL3_TDA:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#108889 <= s_32_4
        fn_state.gs_108889 = s_32_4;
        // N s_32_6: jump b24
        return block_24(state, tracer, fn_state);
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
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // D s_33_4: write-var gs#108888 <= s_33_3
        fn_state.gs_108888 = s_33_3;
        // N s_33_5: jump b22
        return block_22(state, tracer, fn_state);
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
        // D s_35_0: read-var __MDCR_EL3_TDA:u8
        let s_35_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // C s_35_2: const #1u : u8
        let s_35_2: bool = true;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: write-var gs#108887 <= s_35_4
        fn_state.gs_108887 = s_35_4;
        // N s_35_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call ELUsingAArch32(s_36_1)
        let s_36_2: bool = ELUsingAArch32(state, tracer, s_36_1);
        // D s_36_3: not s_36_2
        let s_36_3: bool = !s_36_2;
        // D s_36_4: write-var gs#108886 <= s_36_3
        fn_state.gs_108886 = s_36_3;
        // N s_36_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_37_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_37_1: call __IMPDEF_boolean(s_37_0)
        let s_37_1: bool = u__IMPDEF_boolean(state, tracer, s_37_0);
        // D s_37_2: write-var gs#108885 <= s_37_1
        fn_state.gs_108885 = s_37_1;
        // N s_37_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call EDSCR_read(s_38_0)
        let s_38_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_38_0);
        // S s_38_2: call _get_EDSCR_Type_SDD(s_38_1)
        let s_38_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_38_1);
        // S s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // S s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#108884 <= s_38_6
        fn_state.gs_108884 = s_38_6;
        // N s_38_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #424u : u32
        let s_39_0: u32 = 424;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // C s_39_2: const #2u : u8
        let s_39_2: u8 = 2;
        // D s_39_3: cmp-lt s_39_1 s_39_2
        let s_39_3: bool = ((s_39_1) < (s_39_2));
        // D s_39_4: write-var gs#108883 <= s_39_3
        fn_state.gs_108883 = s_39_3;
        // N s_39_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call Halted(s_40_0)
        let s_40_1: bool = Halted(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b86 b41
        if s_40_1 {
            return block_86(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#108893 <= s_41_0
        fn_state.gs_108893 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#108893:u8
        let s_42_0: bool = fn_state.gs_108893;
        // N s_42_1: branch s_42_0 b85 b43
        if s_42_0 {
            return block_85(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#108894 <= s_43_0
        fn_state.gs_108894 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#108894:u8
        let s_44_0: bool = fn_state.gs_108894;
        // N s_44_1: branch s_44_0 b84 b45
        if s_44_0 {
            return block_84(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#108895 <= s_45_0
        fn_state.gs_108895 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#108895:u8
        let s_46_0: bool = fn_state.gs_108895;
        // N s_46_1: branch s_46_0 b83 b47
        if s_46_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#108896 <= s_47_0
        fn_state.gs_108896 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#108896:u8
        let s_48_0: bool = fn_state.gs_108896;
        // N s_48_1: branch s_48_0 b82 b49
        if s_48_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#108897 <= s_49_0
        fn_state.gs_108897 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#108897:u8
        let s_50_0: bool = fn_state.gs_108897;
        // N s_50_1: branch s_50_0 b81 b51
        if s_50_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call EL2Enabled(s_51_0)
        let s_51_1: bool = EL2Enabled(state, tracer, s_51_0);
        // N s_51_2: branch s_51_1 b80 b52
        if s_51_1 {
            return block_80(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#108898 <= s_52_0
        fn_state.gs_108898 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#108898:u8
        let s_53_0: bool = fn_state.gs_108898;
        // N s_53_1: branch s_53_0 b79 b54
        if s_53_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#108899 <= s_54_0
        fn_state.gs_108899 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#108899:u8
        let s_55_0: bool = fn_state.gs_108899;
        // N s_55_1: branch s_55_0 b78 b56
        if s_55_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call EL2Enabled(s_56_0)
        let s_56_1: bool = EL2Enabled(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b77 b57
        if s_56_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#108900 <= s_57_0
        fn_state.gs_108900 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#108900:u8
        let s_58_0: bool = fn_state.gs_108900;
        // N s_58_1: branch s_58_0 b76 b59
        if s_58_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#108901 <= s_59_0
        fn_state.gs_108901 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#108901:u8
        let s_60_0: bool = fn_state.gs_108901;
        // N s_60_1: branch s_60_0 b75 b61
        if s_60_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #424u : u32
        let s_61_0: u32 = 424;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // C s_61_2: const #2u : u8
        let s_61_2: u8 = 2;
        // D s_61_3: cmp-lt s_61_1 s_61_2
        let s_61_3: bool = ((s_61_1) < (s_61_2));
        // N s_61_4: branch s_61_3 b74 b62
        if s_61_3 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#108902 <= s_62_0
        fn_state.gs_108902 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#108902:u8
        let s_63_0: bool = fn_state.gs_108902;
        // N s_63_1: branch s_63_0 b73 b64
        if s_63_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#108903 <= s_64_0
        fn_state.gs_108903 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#108903:u8
        let s_65_0: bool = fn_state.gs_108903;
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
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call DBGDRAR_read(s_66_0)
        let s_66_1: ProductType5c790c8ef59cc8b2 = DBGDRAR_read(state, tracer, s_66_0);
        // S s_66_2: call __get_DBGDRAR(s_66_1)
        let s_66_2: ProductType5c790c8ef59cc8b2 = u__get_DBGDRAR(state, tracer, s_66_1);
        // D s_66_3: write-var ga#172327 <= s_66_2
        fn_state.ga_172327 = s_66_2;
        // D s_66_4: read-var ga#172327.0:struct
        let s_66_4: u64 = fn_state.ga_172327._0;
        // C s_66_5: const #0s : i
        let s_66_5: i128 = 0;
        // D s_66_6: cast zx s_66_4 -> bv
        let s_66_6: Bits = Bits::new(s_66_4 as u128, 64u16);
        // C s_66_7: const #1s : i64
        let s_66_7: i64 = 1;
        // C s_66_8: cast zx s_66_7 -> i
        let s_66_8: i128 = (i128::try_from(s_66_7).unwrap());
        // C s_66_9: const #31s : i
        let s_66_9: i128 = 31;
        // C s_66_10: add s_66_9 s_66_8
        let s_66_10: i128 = (s_66_9 + s_66_8);
        // D s_66_11: bit-extract s_66_6 s_66_5 s_66_10
        let s_66_11: Bits = (Bits::new(
            ((s_66_6) >> (s_66_5)).value(),
            u16::try_from(s_66_10).unwrap(),
        ));
        // D s_66_12: cast reint s_66_11 -> u32
        let s_66_12: u32 = (s_66_11.value() as u32);
        // D s_66_13: read-var t:i
        let s_66_13: i128 = fn_state.t;
        // D s_66_14: call R_set(s_66_13, s_66_12)
        let s_66_14: () = R_set(state, tracer, s_66_13, s_66_12);
        // N s_66_15: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call Halted(s_67_0)
        let s_67_1: bool = Halted(state, tracer, s_67_0);
        // N s_67_2: branch s_67_1 b72 b68
        if s_67_1 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#108906 <= s_68_0
        fn_state.gs_108906 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#108906:u8
        let s_69_0: bool = fn_state.gs_108906;
        // N s_69_1: branch s_69_0 b71 b70
        if s_69_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #5u : u8
        let s_70_0: u8 = 5;
        // C s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 8u16);
        // C s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (s_70_1.value() as i128);
        // C s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #424u : u32
        let s_70_5: u32 = 424;
        // D s_70_6: read-reg s_70_5:u8
        let s_70_6: u8 = {
            let value = state.read_register::<u8>(s_70_5 as isize);
            tracer.read_register(s_70_5 as isize, value);
            value
        };
        // D s_70_7: call AArch64_AArch32SystemAccessTrap(s_70_6, s_70_4)
        let s_70_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_70_6, s_70_4);
        // N s_70_8: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: panic
        panic!("{:?}", ());
        // N s_71_1: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call EDSCR_read(s_72_0)
        let s_72_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_72_0);
        // S s_72_2: call _get_EDSCR_Type_SDD(s_72_1)
        let s_72_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_72_1);
        // S s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // C s_72_4: const #1u : u8
        let s_72_4: bool = true;
        // C s_72_5: cast zx s_72_4 -> bv
        let s_72_5: Bits = Bits::new(s_72_4 as u128, 1u16);
        // S s_72_6: cmp-eq s_72_3 s_72_5
        let s_72_6: bool = ((s_72_3) == (s_72_5));
        // D s_72_7: write-var gs#108906 <= s_72_6
        fn_state.gs_108906 = s_72_6;
        // N s_72_8: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __MDCR_EL3_TDA:u8
        let s_73_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#108903 <= s_73_4
        fn_state.gs_108903 = s_73_4;
        // N s_73_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #424u : u32
        let s_74_0: u32 = 424;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call ELUsingAArch32(s_74_1)
        let s_74_2: bool = ELUsingAArch32(state, tracer, s_74_1);
        // D s_74_3: not s_74_2
        let s_74_3: bool = !s_74_2;
        // D s_74_4: write-var gs#108902 <= s_74_3
        fn_state.gs_108902 = s_74_3;
        // N s_74_5: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #5u : u8
        let s_75_0: u8 = 5;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // S s_75_5: call AArch32_TakeHypTrapException(s_75_4)
        let s_75_5: () = AArch32_TakeHypTrapException(state, tracer, s_75_4);
        // N s_75_6: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call HDCR_read(s_76_0)
        let s_76_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_76_0);
        // S s_76_2: call _get_HDCR_Type_TDE(s_76_1)
        let s_76_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_76_1);
        // C s_76_3: const #() : ()
        let s_76_3: () = ();
        // S s_76_4: call HDCR_read(s_76_3)
        let s_76_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_76_3);
        // S s_76_5: call _get_HDCR_Type_TDRA(s_76_4)
        let s_76_5: bool = u_get_HDCR_Type_TDRA(state, tracer, s_76_4);
        // S s_76_6: cast zx s_76_2 -> bv
        let s_76_6: Bits = Bits::new(s_76_2 as u128, 1u16);
        // S s_76_7: cast zx s_76_5 -> bv
        let s_76_7: Bits = Bits::new(s_76_5 as u128, 1u16);
        // S s_76_8: cast reint s_76_6 -> u128
        let s_76_8: u128 = (s_76_6.value() as u128);
        // D s_76_9: size-of s_76_6
        let s_76_9: u16 = s_76_6.length();
        // S s_76_10: cast reint s_76_7 -> u128
        let s_76_10: u128 = (s_76_7.value() as u128);
        // D s_76_11: size-of s_76_7
        let s_76_11: u16 = s_76_7.length();
        // D s_76_12: lsl s_76_8 s_76_11
        let s_76_12: u128 = s_76_8 << s_76_11;
        // D s_76_13: or s_76_12 s_76_10
        let s_76_13: u128 = ((s_76_12) | (s_76_10));
        // D s_76_14: add s_76_9 s_76_11
        let s_76_14: u16 = (s_76_9 + s_76_11);
        // D s_76_15: create-bits s_76_13 s_76_14
        let s_76_15: Bits = Bits::new(s_76_13, s_76_14);
        // D s_76_16: cast reint s_76_15 -> u8
        let s_76_16: u8 = (s_76_15.value() as u8);
        // D s_76_17: cast zx s_76_16 -> bv
        let s_76_17: Bits = Bits::new(s_76_16 as u128, 2u16);
        // C s_76_18: const #0u : u8
        let s_76_18: u8 = 0;
        // C s_76_19: cast zx s_76_18 -> bv
        let s_76_19: Bits = Bits::new(s_76_18 as u128, 2u16);
        // D s_76_20: cmp-ne s_76_17 s_76_19
        let s_76_20: bool = ((s_76_17) != (s_76_19));
        // D s_76_21: write-var gs#108901 <= s_76_20
        fn_state.gs_108901 = s_76_20;
        // N s_76_22: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #432u : u32
        let s_77_0: u32 = 432;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call ELUsingAArch32(s_77_1)
        let s_77_2: bool = ELUsingAArch32(state, tracer, s_77_1);
        // D s_77_3: write-var gs#108900 <= s_77_2
        fn_state.gs_108900 = s_77_2;
        // N s_77_4: jump b58
        return block_58(state, tracer, fn_state);
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
        // C s_78_5: const #432u : u32
        let s_78_5: u32 = 432;
        // D s_78_6: read-reg s_78_5:u8
        let s_78_6: u8 = {
            let value = state.read_register::<u8>(s_78_5 as isize);
            tracer.read_register(s_78_5 as isize, value);
            value
        };
        // D s_78_7: call AArch64_AArch32SystemAccessTrap(s_78_6, s_78_4)
        let s_78_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_78_6, s_78_4);
        // N s_78_8: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #104880u : u32
        let s_79_0: u32 = 104880;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_MDCR_EL2_Type_TDE(s_79_1)
        let s_79_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_79_1);
        // C s_79_3: const #104880u : u32
        let s_79_3: u32 = 104880;
        // D s_79_4: read-reg s_79_3:struct
        let s_79_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_3 as isize);
            tracer.read_register(s_79_3 as isize, value);
            value
        };
        // D s_79_5: call _get_MDCR_EL2_Type_TDRA(s_79_4)
        let s_79_5: bool = u_get_MDCR_EL2_Type_TDRA(state, tracer, s_79_4);
        // D s_79_6: cast zx s_79_2 -> bv
        let s_79_6: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_7: cast zx s_79_5 -> bv
        let s_79_7: Bits = Bits::new(s_79_5 as u128, 1u16);
        // D s_79_8: cast reint s_79_6 -> u128
        let s_79_8: u128 = (s_79_6.value() as u128);
        // D s_79_9: size-of s_79_6
        let s_79_9: u16 = s_79_6.length();
        // D s_79_10: cast reint s_79_7 -> u128
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
        // D s_79_21: write-var gs#108899 <= s_79_20
        fn_state.gs_108899 = s_79_20;
        // N s_79_22: jump b55
        return block_55(state, tracer, fn_state);
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
        // D s_80_3: not s_80_2
        let s_80_3: bool = !s_80_2;
        // D s_80_4: write-var gs#108898 <= s_80_3
        fn_state.gs_108898 = s_80_3;
        // N s_80_5: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_81_0: panic
        panic!("{:?}", ());
        // N s_81_1: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var __MDCR_EL3_TDA:u8
        let s_82_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 1u16);
        // C s_82_2: const #1u : u8
        let s_82_2: bool = true;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: write-var gs#108897 <= s_82_4
        fn_state.gs_108897 = s_82_4;
        // N s_82_6: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #424u : u32
        let s_83_0: u32 = 424;
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
        // D s_83_4: write-var gs#108896 <= s_83_3
        fn_state.gs_108896 = s_83_3;
        // N s_83_5: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_84_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_84_1: call __IMPDEF_boolean(s_84_0)
        let s_84_1: bool = u__IMPDEF_boolean(state, tracer, s_84_0);
        // D s_84_2: write-var gs#108895 <= s_84_1
        fn_state.gs_108895 = s_84_1;
        // N s_84_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call EDSCR_read(s_85_0)
        let s_85_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_85_0);
        // S s_85_2: call _get_EDSCR_Type_SDD(s_85_1)
        let s_85_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_85_1);
        // S s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // C s_85_4: const #1u : u8
        let s_85_4: bool = true;
        // C s_85_5: cast zx s_85_4 -> bv
        let s_85_5: Bits = Bits::new(s_85_4 as u128, 1u16);
        // S s_85_6: cmp-eq s_85_3 s_85_5
        let s_85_6: bool = ((s_85_3) == (s_85_5));
        // D s_85_7: write-var gs#108894 <= s_85_6
        fn_state.gs_108894 = s_85_6;
        // N s_85_8: jump b44
        return block_44(state, tracer, fn_state);
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
        // C s_86_2: const #2u : u8
        let s_86_2: u8 = 2;
        // D s_86_3: cmp-lt s_86_1 s_86_2
        let s_86_3: bool = ((s_86_1) < (s_86_2));
        // D s_86_4: write-var gs#108893 <= s_86_3
        fn_state.gs_108893 = s_86_3;
        // N s_86_5: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #() : ()
        let s_87_0: () = ();
        // S s_87_1: call Halted(s_87_0)
        let s_87_1: bool = Halted(state, tracer, s_87_0);
        // N s_87_2: branch s_87_1 b173 b88
        if s_87_1 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#108907 <= s_88_0
        fn_state.gs_108907 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#108907:u8
        let s_89_0: bool = fn_state.gs_108907;
        // N s_89_1: branch s_89_0 b172 b90
        if s_89_0 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#108908 <= s_90_0
        fn_state.gs_108908 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#108908:u8
        let s_91_0: bool = fn_state.gs_108908;
        // N s_91_1: branch s_91_0 b171 b92
        if s_91_0 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#108909 <= s_92_0
        fn_state.gs_108909 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#108909:u8
        let s_93_0: bool = fn_state.gs_108909;
        // N s_93_1: branch s_93_0 b170 b94
        if s_93_0 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#108910 <= s_94_0
        fn_state.gs_108910 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#108910:u8
        let s_95_0: bool = fn_state.gs_108910;
        // N s_95_1: branch s_95_0 b169 b96
        if s_95_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#108911 <= s_96_0
        fn_state.gs_108911 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#108911:u8
        let s_97_0: bool = fn_state.gs_108911;
        // N s_97_1: branch s_97_0 b168 b98
        if s_97_0 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #440u : u32
        let s_98_0: u32 = 440;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call ELUsingAArch32(s_98_1)
        let s_98_2: bool = ELUsingAArch32(state, tracer, s_98_1);
        // D s_98_3: not s_98_2
        let s_98_3: bool = !s_98_2;
        // N s_98_4: branch s_98_3 b167 b99
        if s_98_3 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#108912 <= s_99_0
        fn_state.gs_108912 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#108912:u8
        let s_100_0: bool = fn_state.gs_108912;
        // N s_100_1: branch s_100_0 b158 b101
        if s_100_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #440u : u32
        let s_101_0: u32 = 440;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call ELUsingAArch32(s_101_1)
        let s_101_2: bool = ELUsingAArch32(state, tracer, s_101_1);
        // N s_101_3: branch s_101_2 b157 b102
        if s_101_2 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#108913 <= s_102_0
        fn_state.gs_108913 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#108913:u8
        let s_103_0: bool = fn_state.gs_108913;
        // N s_103_1: branch s_103_0 b140 b104
        if s_103_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #() : ()
        let s_104_0: () = ();
        // S s_104_1: call EL2Enabled(s_104_0)
        let s_104_1: bool = EL2Enabled(state, tracer, s_104_0);
        // N s_104_2: branch s_104_1 b139 b105
        if s_104_1 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#108914 <= s_105_0
        fn_state.gs_108914 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#108914:u8
        let s_106_0: bool = fn_state.gs_108914;
        // N s_106_1: branch s_106_0 b135 b107
        if s_106_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#108916 <= s_107_0
        fn_state.gs_108916 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#108916:u8
        let s_108_0: bool = fn_state.gs_108916;
        // N s_108_1: branch s_108_0 b134 b109
        if s_108_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EL2Enabled(s_109_0)
        let s_109_1: bool = EL2Enabled(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b133 b110
        if s_109_1 {
            return block_133(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#108917 <= s_110_0
        fn_state.gs_108917 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#108917:u8
        let s_111_0: bool = fn_state.gs_108917;
        // N s_111_1: branch s_111_0 b129 b112
        if s_111_0 {
            return block_129(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#108919 <= s_112_0
        fn_state.gs_108919 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#108919:u8
        let s_113_0: bool = fn_state.gs_108919;
        // N s_113_1: branch s_113_0 b128 b114
        if s_113_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #424u : u32
        let s_114_0: u32 = 424;
        // D s_114_1: read-reg s_114_0:u8
        let s_114_1: u8 = {
            let value = state.read_register::<u8>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // C s_114_2: const #2u : u8
        let s_114_2: u8 = 2;
        // D s_114_3: cmp-lt s_114_1 s_114_2
        let s_114_3: bool = ((s_114_1) < (s_114_2));
        // N s_114_4: branch s_114_3 b127 b115
        if s_114_3 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#108920 <= s_115_0
        fn_state.gs_108920 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#108920:u8
        let s_116_0: bool = fn_state.gs_108920;
        // N s_116_1: branch s_116_0 b126 b117
        if s_116_0 {
            return block_126(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#108921 <= s_117_0
        fn_state.gs_108921 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#108921:u8
        let s_118_0: bool = fn_state.gs_108921;
        // N s_118_1: branch s_118_0 b120 b119
        if s_118_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call DBGDRAR_read(s_119_0)
        let s_119_1: ProductType5c790c8ef59cc8b2 = DBGDRAR_read(state, tracer, s_119_0);
        // S s_119_2: call __get_DBGDRAR(s_119_1)
        let s_119_2: ProductType5c790c8ef59cc8b2 = u__get_DBGDRAR(
            state,
            tracer,
            s_119_1,
        );
        // D s_119_3: write-var ga#172285 <= s_119_2
        fn_state.ga_172285 = s_119_2;
        // D s_119_4: read-var ga#172285.0:struct
        let s_119_4: u64 = fn_state.ga_172285._0;
        // C s_119_5: const #0s : i
        let s_119_5: i128 = 0;
        // D s_119_6: cast zx s_119_4 -> bv
        let s_119_6: Bits = Bits::new(s_119_4 as u128, 64u16);
        // C s_119_7: const #1s : i64
        let s_119_7: i64 = 1;
        // C s_119_8: cast zx s_119_7 -> i
        let s_119_8: i128 = (i128::try_from(s_119_7).unwrap());
        // C s_119_9: const #31s : i
        let s_119_9: i128 = 31;
        // C s_119_10: add s_119_9 s_119_8
        let s_119_10: i128 = (s_119_9 + s_119_8);
        // D s_119_11: bit-extract s_119_6 s_119_5 s_119_10
        let s_119_11: Bits = (Bits::new(
            ((s_119_6) >> (s_119_5)).value(),
            u16::try_from(s_119_10).unwrap(),
        ));
        // D s_119_12: cast reint s_119_11 -> u32
        let s_119_12: u32 = (s_119_11.value() as u32);
        // D s_119_13: read-var t:i
        let s_119_13: i128 = fn_state.t;
        // D s_119_14: call R_set(s_119_13, s_119_12)
        let s_119_14: () = R_set(state, tracer, s_119_13, s_119_12);
        // N s_119_15: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #() : ()
        let s_120_0: () = ();
        // S s_120_1: call Halted(s_120_0)
        let s_120_1: bool = Halted(state, tracer, s_120_0);
        // N s_120_2: branch s_120_1 b125 b121
        if s_120_1 {
            return block_125(state, tracer, fn_state);
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
        // D s_121_1: write-var gs#108924 <= s_121_0
        fn_state.gs_108924 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#108924:u8
        let s_122_0: bool = fn_state.gs_108924;
        // N s_122_1: branch s_122_0 b124 b123
        if s_122_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #5u : u8
        let s_123_0: u8 = 5;
        // C s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 8u16);
        // C s_123_2: cast zx s_123_1 -> i
        let s_123_2: i128 = (s_123_1.value() as i128);
        // C s_123_3: cast reint s_123_2 -> i64
        let s_123_3: i64 = (s_123_2 as i64);
        // C s_123_4: cast zx s_123_3 -> i
        let s_123_4: i128 = (i128::try_from(s_123_3).unwrap());
        // C s_123_5: const #424u : u32
        let s_123_5: u32 = 424;
        // D s_123_6: read-reg s_123_5:u8
        let s_123_6: u8 = {
            let value = state.read_register::<u8>(s_123_5 as isize);
            tracer.read_register(s_123_5 as isize, value);
            value
        };
        // D s_123_7: call AArch64_AArch32SystemAccessTrap(s_123_6, s_123_4)
        let s_123_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_123_6,
            s_123_4,
        );
        // N s_123_8: return
        return;
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_124_0: panic
        panic!("{:?}", ());
        // N s_124_1: return
        return;
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #() : ()
        let s_125_0: () = ();
        // S s_125_1: call EDSCR_read(s_125_0)
        let s_125_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_125_0);
        // S s_125_2: call _get_EDSCR_Type_SDD(s_125_1)
        let s_125_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_125_1);
        // S s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // C s_125_4: const #1u : u8
        let s_125_4: bool = true;
        // C s_125_5: cast zx s_125_4 -> bv
        let s_125_5: Bits = Bits::new(s_125_4 as u128, 1u16);
        // S s_125_6: cmp-eq s_125_3 s_125_5
        let s_125_6: bool = ((s_125_3) == (s_125_5));
        // D s_125_7: write-var gs#108924 <= s_125_6
        fn_state.gs_108924 = s_125_6;
        // N s_125_8: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var __MDCR_EL3_TDA:u8
        let s_126_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 1u16);
        // C s_126_2: const #1u : u8
        let s_126_2: bool = true;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: write-var gs#108921 <= s_126_4
        fn_state.gs_108921 = s_126_4;
        // N s_126_6: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #424u : u32
        let s_127_0: u32 = 424;
        // D s_127_1: read-reg s_127_0:u8
        let s_127_1: u8 = {
            let value = state.read_register::<u8>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // D s_127_2: call ELUsingAArch32(s_127_1)
        let s_127_2: bool = ELUsingAArch32(state, tracer, s_127_1);
        // D s_127_3: not s_127_2
        let s_127_3: bool = !s_127_2;
        // D s_127_4: write-var gs#108920 <= s_127_3
        fn_state.gs_108920 = s_127_3;
        // N s_127_5: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #5u : u8
        let s_128_0: u8 = 5;
        // C s_128_1: cast zx s_128_0 -> bv
        let s_128_1: Bits = Bits::new(s_128_0 as u128, 8u16);
        // C s_128_2: cast zx s_128_1 -> i
        let s_128_2: i128 = (s_128_1.value() as i128);
        // C s_128_3: cast reint s_128_2 -> i64
        let s_128_3: i64 = (s_128_2 as i64);
        // C s_128_4: cast zx s_128_3 -> i
        let s_128_4: i128 = (i128::try_from(s_128_3).unwrap());
        // S s_128_5: call AArch32_TakeHypTrapException(s_128_4)
        let s_128_5: () = AArch32_TakeHypTrapException(state, tracer, s_128_4);
        // N s_128_6: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #() : ()
        let s_129_0: () = ();
        // S s_129_1: call HCR_read(s_129_0)
        let s_129_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_129_0);
        // S s_129_2: call _get_HCR_Type_TGE(s_129_1)
        let s_129_2: bool = u_get_HCR_Type_TGE(state, tracer, s_129_1);
        // S s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // C s_129_4: const #1u : u8
        let s_129_4: bool = true;
        // C s_129_5: cast zx s_129_4 -> bv
        let s_129_5: Bits = Bits::new(s_129_4 as u128, 1u16);
        // S s_129_6: cmp-eq s_129_3 s_129_5
        let s_129_6: bool = ((s_129_3) == (s_129_5));
        // N s_129_7: branch s_129_6 b132 b130
        if s_129_6 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #() : ()
        let s_130_0: () = ();
        // S s_130_1: call HDCR_read(s_130_0)
        let s_130_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_130_0);
        // S s_130_2: call _get_HDCR_Type_TDE(s_130_1)
        let s_130_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_130_1);
        // C s_130_3: const #() : ()
        let s_130_3: () = ();
        // S s_130_4: call HDCR_read(s_130_3)
        let s_130_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_130_3);
        // S s_130_5: call _get_HDCR_Type_TDRA(s_130_4)
        let s_130_5: bool = u_get_HDCR_Type_TDRA(state, tracer, s_130_4);
        // S s_130_6: cast zx s_130_2 -> bv
        let s_130_6: Bits = Bits::new(s_130_2 as u128, 1u16);
        // S s_130_7: cast zx s_130_5 -> bv
        let s_130_7: Bits = Bits::new(s_130_5 as u128, 1u16);
        // S s_130_8: cast reint s_130_6 -> u128
        let s_130_8: u128 = (s_130_6.value() as u128);
        // D s_130_9: size-of s_130_6
        let s_130_9: u16 = s_130_6.length();
        // S s_130_10: cast reint s_130_7 -> u128
        let s_130_10: u128 = (s_130_7.value() as u128);
        // D s_130_11: size-of s_130_7
        let s_130_11: u16 = s_130_7.length();
        // D s_130_12: lsl s_130_8 s_130_11
        let s_130_12: u128 = s_130_8 << s_130_11;
        // D s_130_13: or s_130_12 s_130_10
        let s_130_13: u128 = ((s_130_12) | (s_130_10));
        // D s_130_14: add s_130_9 s_130_11
        let s_130_14: u16 = (s_130_9 + s_130_11);
        // D s_130_15: create-bits s_130_13 s_130_14
        let s_130_15: Bits = Bits::new(s_130_13, s_130_14);
        // D s_130_16: cast reint s_130_15 -> u8
        let s_130_16: u8 = (s_130_15.value() as u8);
        // D s_130_17: cast zx s_130_16 -> bv
        let s_130_17: Bits = Bits::new(s_130_16 as u128, 2u16);
        // C s_130_18: const #0u : u8
        let s_130_18: u8 = 0;
        // C s_130_19: cast zx s_130_18 -> bv
        let s_130_19: Bits = Bits::new(s_130_18 as u128, 2u16);
        // D s_130_20: cmp-ne s_130_17 s_130_19
        let s_130_20: bool = ((s_130_17) != (s_130_19));
        // D s_130_21: write-var gs#108918 <= s_130_20
        fn_state.gs_108918 = s_130_20;
        // N s_130_22: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#108918:u8
        let s_131_0: bool = fn_state.gs_108918;
        // D s_131_1: write-var gs#108919 <= s_131_0
        fn_state.gs_108919 = s_131_0;
        // N s_131_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #1u : u8
        let s_132_0: bool = true;
        // D s_132_1: write-var gs#108918 <= s_132_0
        fn_state.gs_108918 = s_132_0;
        // N s_132_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #432u : u32
        let s_133_0: u32 = 432;
        // D s_133_1: read-reg s_133_0:u8
        let s_133_1: u8 = {
            let value = state.read_register::<u8>(s_133_0 as isize);
            tracer.read_register(s_133_0 as isize, value);
            value
        };
        // D s_133_2: call ELUsingAArch32(s_133_1)
        let s_133_2: bool = ELUsingAArch32(state, tracer, s_133_1);
        // D s_133_3: write-var gs#108917 <= s_133_2
        fn_state.gs_108917 = s_133_2;
        // N s_133_4: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #5u : u8
        let s_134_0: u8 = 5;
        // C s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 8u16);
        // C s_134_2: cast zx s_134_1 -> i
        let s_134_2: i128 = (s_134_1.value() as i128);
        // C s_134_3: cast reint s_134_2 -> i64
        let s_134_3: i64 = (s_134_2 as i64);
        // C s_134_4: cast zx s_134_3 -> i
        let s_134_4: i128 = (i128::try_from(s_134_3).unwrap());
        // C s_134_5: const #432u : u32
        let s_134_5: u32 = 432;
        // D s_134_6: read-reg s_134_5:u8
        let s_134_6: u8 = {
            let value = state.read_register::<u8>(s_134_5 as isize);
            tracer.read_register(s_134_5 as isize, value);
            value
        };
        // D s_134_7: call AArch64_AArch32SystemAccessTrap(s_134_6, s_134_4)
        let s_134_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_134_6,
            s_134_4,
        );
        // N s_134_8: return
        return;
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #102552u : u32
        let s_135_0: u32 = 102552;
        // D s_135_1: read-reg s_135_0:struct
        let s_135_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_135_0 as isize);
            tracer.read_register(s_135_0 as isize, value);
            value
        };
        // D s_135_2: call _get_HCR_EL2_Type_TGE(s_135_1)
        let s_135_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_135_1);
        // D s_135_3: cast zx s_135_2 -> bv
        let s_135_3: Bits = Bits::new(s_135_2 as u128, 1u16);
        // C s_135_4: const #1u : u8
        let s_135_4: bool = true;
        // C s_135_5: cast zx s_135_4 -> bv
        let s_135_5: Bits = Bits::new(s_135_4 as u128, 1u16);
        // D s_135_6: cmp-eq s_135_3 s_135_5
        let s_135_6: bool = ((s_135_3) == (s_135_5));
        // N s_135_7: branch s_135_6 b138 b136
        if s_135_6 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #104880u : u32
        let s_136_0: u32 = 104880;
        // D s_136_1: read-reg s_136_0:struct
        let s_136_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_136_0 as isize);
            tracer.read_register(s_136_0 as isize, value);
            value
        };
        // D s_136_2: call _get_MDCR_EL2_Type_TDE(s_136_1)
        let s_136_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_136_1);
        // C s_136_3: const #104880u : u32
        let s_136_3: u32 = 104880;
        // D s_136_4: read-reg s_136_3:struct
        let s_136_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_136_3 as isize);
            tracer.read_register(s_136_3 as isize, value);
            value
        };
        // D s_136_5: call _get_MDCR_EL2_Type_TDRA(s_136_4)
        let s_136_5: bool = u_get_MDCR_EL2_Type_TDRA(state, tracer, s_136_4);
        // D s_136_6: cast zx s_136_2 -> bv
        let s_136_6: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_7: cast zx s_136_5 -> bv
        let s_136_7: Bits = Bits::new(s_136_5 as u128, 1u16);
        // D s_136_8: cast reint s_136_6 -> u128
        let s_136_8: u128 = (s_136_6.value() as u128);
        // D s_136_9: size-of s_136_6
        let s_136_9: u16 = s_136_6.length();
        // D s_136_10: cast reint s_136_7 -> u128
        let s_136_10: u128 = (s_136_7.value() as u128);
        // D s_136_11: size-of s_136_7
        let s_136_11: u16 = s_136_7.length();
        // D s_136_12: lsl s_136_8 s_136_11
        let s_136_12: u128 = s_136_8 << s_136_11;
        // D s_136_13: or s_136_12 s_136_10
        let s_136_13: u128 = ((s_136_12) | (s_136_10));
        // D s_136_14: add s_136_9 s_136_11
        let s_136_14: u16 = (s_136_9 + s_136_11);
        // D s_136_15: create-bits s_136_13 s_136_14
        let s_136_15: Bits = Bits::new(s_136_13, s_136_14);
        // D s_136_16: cast reint s_136_15 -> u8
        let s_136_16: u8 = (s_136_15.value() as u8);
        // D s_136_17: cast zx s_136_16 -> bv
        let s_136_17: Bits = Bits::new(s_136_16 as u128, 2u16);
        // C s_136_18: const #0u : u8
        let s_136_18: u8 = 0;
        // C s_136_19: cast zx s_136_18 -> bv
        let s_136_19: Bits = Bits::new(s_136_18 as u128, 2u16);
        // D s_136_20: cmp-ne s_136_17 s_136_19
        let s_136_20: bool = ((s_136_17) != (s_136_19));
        // D s_136_21: write-var gs#108915 <= s_136_20
        fn_state.gs_108915 = s_136_20;
        // N s_136_22: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#108915:u8
        let s_137_0: bool = fn_state.gs_108915;
        // D s_137_1: write-var gs#108916 <= s_137_0
        fn_state.gs_108916 = s_137_0;
        // N s_137_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #1u : u8
        let s_138_0: bool = true;
        // D s_138_1: write-var gs#108915 <= s_138_0
        fn_state.gs_108915 = s_138_0;
        // N s_138_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #432u : u32
        let s_139_0: u32 = 432;
        // D s_139_1: read-reg s_139_0:u8
        let s_139_1: u8 = {
            let value = state.read_register::<u8>(s_139_0 as isize);
            tracer.read_register(s_139_0 as isize, value);
            value
        };
        // D s_139_2: call ELUsingAArch32(s_139_1)
        let s_139_2: bool = ELUsingAArch32(state, tracer, s_139_1);
        // D s_139_3: not s_139_2
        let s_139_3: bool = !s_139_2;
        // D s_139_4: write-var gs#108914 <= s_139_3
        fn_state.gs_108914 = s_139_3;
        // N s_139_5: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #() : ()
        let s_140_0: () = ();
        // S s_140_1: call EL2Enabled(s_140_0)
        let s_140_1: bool = EL2Enabled(state, tracer, s_140_0);
        // N s_140_2: branch s_140_1 b156 b141
        if s_140_1 {
            return block_156(state, tracer, fn_state);
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
        // D s_141_1: write-var gs#108925 <= s_141_0
        fn_state.gs_108925 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#108925:u8
        let s_142_0: bool = fn_state.gs_108925;
        // N s_142_1: branch s_142_0 b155 b143
        if s_142_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#108926 <= s_143_0
        fn_state.gs_108926 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#108926:u8
        let s_144_0: bool = fn_state.gs_108926;
        // N s_144_1: branch s_144_0 b154 b145
        if s_144_0 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #() : ()
        let s_145_0: () = ();
        // S s_145_1: call EL2Enabled(s_145_0)
        let s_145_1: bool = EL2Enabled(state, tracer, s_145_0);
        // N s_145_2: branch s_145_1 b153 b146
        if s_145_1 {
            return block_153(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#108927 <= s_146_0
        fn_state.gs_108927 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#108927:u8
        let s_147_0: bool = fn_state.gs_108927;
        // N s_147_1: branch s_147_0 b152 b148
        if s_147_0 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#108928 <= s_148_0
        fn_state.gs_108928 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#108928:u8
        let s_149_0: bool = fn_state.gs_108928;
        // N s_149_1: branch s_149_0 b151 b150
        if s_149_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_150_0: panic
        panic!("{:?}", ());
        // N s_150_1: return
        return;
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: u8 = 0;
        // C s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 8u16);
        // C s_151_2: cast zx s_151_1 -> i
        let s_151_2: i128 = (s_151_1.value() as i128);
        // C s_151_3: cast reint s_151_2 -> i64
        let s_151_3: i64 = (s_151_2 as i64);
        // C s_151_4: cast zx s_151_3 -> i
        let s_151_4: i128 = (i128::try_from(s_151_3).unwrap());
        // S s_151_5: call AArch32_TakeHypTrapException(s_151_4)
        let s_151_5: () = AArch32_TakeHypTrapException(state, tracer, s_151_4);
        // N s_151_6: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #() : ()
        let s_152_0: () = ();
        // S s_152_1: call HCR_read(s_152_0)
        let s_152_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_152_0);
        // S s_152_2: call _get_HCR_Type_TGE(s_152_1)
        let s_152_2: bool = u_get_HCR_Type_TGE(state, tracer, s_152_1);
        // S s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 1u16);
        // C s_152_4: const #1u : u8
        let s_152_4: bool = true;
        // C s_152_5: cast zx s_152_4 -> bv
        let s_152_5: Bits = Bits::new(s_152_4 as u128, 1u16);
        // S s_152_6: cmp-eq s_152_3 s_152_5
        let s_152_6: bool = ((s_152_3) == (s_152_5));
        // D s_152_7: write-var gs#108928 <= s_152_6
        fn_state.gs_108928 = s_152_6;
        // N s_152_8: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #432u : u32
        let s_153_0: u32 = 432;
        // D s_153_1: read-reg s_153_0:u8
        let s_153_1: u8 = {
            let value = state.read_register::<u8>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // D s_153_2: call ELUsingAArch32(s_153_1)
        let s_153_2: bool = ELUsingAArch32(state, tracer, s_153_1);
        // D s_153_3: write-var gs#108927 <= s_153_2
        fn_state.gs_108927 = s_153_2;
        // N s_153_4: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #5u : u8
        let s_154_0: u8 = 5;
        // C s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 8u16);
        // C s_154_2: cast zx s_154_1 -> i
        let s_154_2: i128 = (s_154_1.value() as i128);
        // C s_154_3: cast reint s_154_2 -> i64
        let s_154_3: i64 = (s_154_2 as i64);
        // C s_154_4: cast zx s_154_3 -> i
        let s_154_4: i128 = (i128::try_from(s_154_3).unwrap());
        // C s_154_5: const #432u : u32
        let s_154_5: u32 = 432;
        // D s_154_6: read-reg s_154_5:u8
        let s_154_6: u8 = {
            let value = state.read_register::<u8>(s_154_5 as isize);
            tracer.read_register(s_154_5 as isize, value);
            value
        };
        // D s_154_7: call AArch64_AArch32SystemAccessTrap(s_154_6, s_154_4)
        let s_154_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_154_6,
            s_154_4,
        );
        // N s_154_8: return
        return;
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #102552u : u32
        let s_155_0: u32 = 102552;
        // D s_155_1: read-reg s_155_0:struct
        let s_155_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_155_0 as isize);
            tracer.read_register(s_155_0 as isize, value);
            value
        };
        // D s_155_2: call _get_HCR_EL2_Type_TGE(s_155_1)
        let s_155_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_155_1);
        // D s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 1u16);
        // C s_155_4: const #1u : u8
        let s_155_4: bool = true;
        // C s_155_5: cast zx s_155_4 -> bv
        let s_155_5: Bits = Bits::new(s_155_4 as u128, 1u16);
        // D s_155_6: cmp-eq s_155_3 s_155_5
        let s_155_6: bool = ((s_155_3) == (s_155_5));
        // D s_155_7: write-var gs#108926 <= s_155_6
        fn_state.gs_108926 = s_155_6;
        // N s_155_8: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #432u : u32
        let s_156_0: u32 = 432;
        // D s_156_1: read-reg s_156_0:u8
        let s_156_1: u8 = {
            let value = state.read_register::<u8>(s_156_0 as isize);
            tracer.read_register(s_156_0 as isize, value);
            value
        };
        // D s_156_2: call ELUsingAArch32(s_156_1)
        let s_156_2: bool = ELUsingAArch32(state, tracer, s_156_1);
        // D s_156_3: not s_156_2
        let s_156_3: bool = !s_156_2;
        // D s_156_4: write-var gs#108925 <= s_156_3
        fn_state.gs_108925 = s_156_3;
        // N s_156_5: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var __DBGDSCRext_UDCCdis:u8
        let s_157_0: bool = fn_state.u__DBGDSCRext_UDCCdis;
        // D s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 1u16);
        // C s_157_2: const #1u : u8
        let s_157_2: bool = true;
        // C s_157_3: cast zx s_157_2 -> bv
        let s_157_3: Bits = Bits::new(s_157_2 as u128, 1u16);
        // D s_157_4: cmp-eq s_157_1 s_157_3
        let s_157_4: bool = ((s_157_1) == (s_157_3));
        // D s_157_5: write-var gs#108913 <= s_157_4
        fn_state.gs_108913 = s_157_4;
        // N s_157_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #() : ()
        let s_158_0: () = ();
        // S s_158_1: call EL2Enabled(s_158_0)
        let s_158_1: bool = EL2Enabled(state, tracer, s_158_0);
        // N s_158_2: branch s_158_1 b166 b159
        if s_158_1 {
            return block_166(state, tracer, fn_state);
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
        // D s_159_1: write-var gs#108929 <= s_159_0
        fn_state.gs_108929 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#108929:u8
        let s_160_0: bool = fn_state.gs_108929;
        // N s_160_1: branch s_160_0 b165 b161
        if s_160_0 {
            return block_165(state, tracer, fn_state);
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
        // D s_161_1: write-var gs#108930 <= s_161_0
        fn_state.gs_108930 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#108930:u8
        let s_162_0: bool = fn_state.gs_108930;
        // N s_162_1: branch s_162_0 b164 b163
        if s_162_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #5u : u8
        let s_163_0: u8 = 5;
        // C s_163_1: cast zx s_163_0 -> bv
        let s_163_1: Bits = Bits::new(s_163_0 as u128, 8u16);
        // C s_163_2: cast zx s_163_1 -> i
        let s_163_2: i128 = (s_163_1.value() as i128);
        // C s_163_3: cast reint s_163_2 -> i64
        let s_163_3: i64 = (s_163_2 as i64);
        // C s_163_4: cast zx s_163_3 -> i
        let s_163_4: i128 = (i128::try_from(s_163_3).unwrap());
        // C s_163_5: const #440u : u32
        let s_163_5: u32 = 440;
        // D s_163_6: read-reg s_163_5:u8
        let s_163_6: u8 = {
            let value = state.read_register::<u8>(s_163_5 as isize);
            tracer.read_register(s_163_5 as isize, value);
            value
        };
        // D s_163_7: call AArch64_AArch32SystemAccessTrap(s_163_6, s_163_4)
        let s_163_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_163_6,
            s_163_4,
        );
        // N s_163_8: return
        return;
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #5u : u8
        let s_164_0: u8 = 5;
        // C s_164_1: cast zx s_164_0 -> bv
        let s_164_1: Bits = Bits::new(s_164_0 as u128, 8u16);
        // C s_164_2: cast zx s_164_1 -> i
        let s_164_2: i128 = (s_164_1.value() as i128);
        // C s_164_3: cast reint s_164_2 -> i64
        let s_164_3: i64 = (s_164_2 as i64);
        // C s_164_4: cast zx s_164_3 -> i
        let s_164_4: i128 = (i128::try_from(s_164_3).unwrap());
        // C s_164_5: const #432u : u32
        let s_164_5: u32 = 432;
        // D s_164_6: read-reg s_164_5:u8
        let s_164_6: u8 = {
            let value = state.read_register::<u8>(s_164_5 as isize);
            tracer.read_register(s_164_5 as isize, value);
            value
        };
        // D s_164_7: call AArch64_AArch32SystemAccessTrap(s_164_6, s_164_4)
        let s_164_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_164_6,
            s_164_4,
        );
        // N s_164_8: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #102552u : u32
        let s_165_0: u32 = 102552;
        // D s_165_1: read-reg s_165_0:struct
        let s_165_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_165_0 as isize);
            tracer.read_register(s_165_0 as isize, value);
            value
        };
        // D s_165_2: call _get_HCR_EL2_Type_TGE(s_165_1)
        let s_165_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_165_1);
        // D s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // C s_165_4: const #1u : u8
        let s_165_4: bool = true;
        // C s_165_5: cast zx s_165_4 -> bv
        let s_165_5: Bits = Bits::new(s_165_4 as u128, 1u16);
        // D s_165_6: cmp-eq s_165_3 s_165_5
        let s_165_6: bool = ((s_165_3) == (s_165_5));
        // D s_165_7: write-var gs#108930 <= s_165_6
        fn_state.gs_108930 = s_165_6;
        // N s_165_8: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #432u : u32
        let s_166_0: u32 = 432;
        // D s_166_1: read-reg s_166_0:u8
        let s_166_1: u8 = {
            let value = state.read_register::<u8>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // D s_166_2: call ELUsingAArch32(s_166_1)
        let s_166_2: bool = ELUsingAArch32(state, tracer, s_166_1);
        // D s_166_3: not s_166_2
        let s_166_3: bool = !s_166_2;
        // D s_166_4: write-var gs#108929 <= s_166_3
        fn_state.gs_108929 = s_166_3;
        // N s_166_5: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var __MDSCR_EL1_TDCC:u8
        let s_167_0: bool = fn_state.u__MDSCR_EL1_TDCC;
        // D s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 1u16);
        // C s_167_2: const #1u : u8
        let s_167_2: bool = true;
        // C s_167_3: cast zx s_167_2 -> bv
        let s_167_3: Bits = Bits::new(s_167_2 as u128, 1u16);
        // D s_167_4: cmp-eq s_167_1 s_167_3
        let s_167_4: bool = ((s_167_1) == (s_167_3));
        // D s_167_5: write-var gs#108912 <= s_167_4
        fn_state.gs_108912 = s_167_4;
        // N s_167_6: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_168_0: panic
        panic!("{:?}", ());
        // N s_168_1: return
        return;
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var __MDCR_EL3_TDA:u8
        let s_169_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 1u16);
        // C s_169_2: const #1u : u8
        let s_169_2: bool = true;
        // C s_169_3: cast zx s_169_2 -> bv
        let s_169_3: Bits = Bits::new(s_169_2 as u128, 1u16);
        // D s_169_4: cmp-eq s_169_1 s_169_3
        let s_169_4: bool = ((s_169_1) == (s_169_3));
        // D s_169_5: write-var gs#108911 <= s_169_4
        fn_state.gs_108911 = s_169_4;
        // N s_169_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #424u : u32
        let s_170_0: u32 = 424;
        // D s_170_1: read-reg s_170_0:u8
        let s_170_1: u8 = {
            let value = state.read_register::<u8>(s_170_0 as isize);
            tracer.read_register(s_170_0 as isize, value);
            value
        };
        // D s_170_2: call ELUsingAArch32(s_170_1)
        let s_170_2: bool = ELUsingAArch32(state, tracer, s_170_1);
        // D s_170_3: not s_170_2
        let s_170_3: bool = !s_170_2;
        // D s_170_4: write-var gs#108910 <= s_170_3
        fn_state.gs_108910 = s_170_3;
        // N s_170_5: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_171_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_171_1: call __IMPDEF_boolean(s_171_0)
        let s_171_1: bool = u__IMPDEF_boolean(state, tracer, s_171_0);
        // D s_171_2: write-var gs#108909 <= s_171_1
        fn_state.gs_108909 = s_171_1;
        // N s_171_3: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #() : ()
        let s_172_0: () = ();
        // S s_172_1: call EDSCR_read(s_172_0)
        let s_172_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_172_0);
        // S s_172_2: call _get_EDSCR_Type_SDD(s_172_1)
        let s_172_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_172_1);
        // S s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 1u16);
        // C s_172_4: const #1u : u8
        let s_172_4: bool = true;
        // C s_172_5: cast zx s_172_4 -> bv
        let s_172_5: Bits = Bits::new(s_172_4 as u128, 1u16);
        // S s_172_6: cmp-eq s_172_3 s_172_5
        let s_172_6: bool = ((s_172_3) == (s_172_5));
        // D s_172_7: write-var gs#108908 <= s_172_6
        fn_state.gs_108908 = s_172_6;
        // N s_172_8: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #424u : u32
        let s_173_0: u32 = 424;
        // D s_173_1: read-reg s_173_0:u8
        let s_173_1: u8 = {
            let value = state.read_register::<u8>(s_173_0 as isize);
            tracer.read_register(s_173_0 as isize, value);
            value
        };
        // C s_173_2: const #2u : u8
        let s_173_2: u8 = 2;
        // D s_173_3: cmp-lt s_173_1 s_173_2
        let s_173_3: bool = ((s_173_1) < (s_173_2));
        // D s_173_4: write-var gs#108907 <= s_173_3
        fn_state.gs_108907 = s_173_3;
        // N s_173_5: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #() : ()
        let s_174_0: () = ();
        // S s_174_1: call DBGDRAR_read(s_174_0)
        let s_174_1: ProductType5c790c8ef59cc8b2 = DBGDRAR_read(state, tracer, s_174_0);
        // S s_174_2: call __get_DBGDRAR(s_174_1)
        let s_174_2: ProductType5c790c8ef59cc8b2 = u__get_DBGDRAR(
            state,
            tracer,
            s_174_1,
        );
        // D s_174_3: write-var ga#172213 <= s_174_2
        fn_state.ga_172213 = s_174_2;
        // D s_174_4: read-var ga#172213.0:struct
        let s_174_4: u64 = fn_state.ga_172213._0;
        // C s_174_5: const #0s : i
        let s_174_5: i128 = 0;
        // D s_174_6: cast zx s_174_4 -> bv
        let s_174_6: Bits = Bits::new(s_174_4 as u128, 64u16);
        // C s_174_7: const #1s : i64
        let s_174_7: i64 = 1;
        // C s_174_8: cast zx s_174_7 -> i
        let s_174_8: i128 = (i128::try_from(s_174_7).unwrap());
        // C s_174_9: const #31s : i
        let s_174_9: i128 = 31;
        // C s_174_10: add s_174_9 s_174_8
        let s_174_10: i128 = (s_174_9 + s_174_8);
        // D s_174_11: bit-extract s_174_6 s_174_5 s_174_10
        let s_174_11: Bits = (Bits::new(
            ((s_174_6) >> (s_174_5)).value(),
            u16::try_from(s_174_10).unwrap(),
        ));
        // D s_174_12: cast reint s_174_11 -> u32
        let s_174_12: u32 = (s_174_11.value() as u32);
        // D s_174_13: read-var t:i
        let s_174_13: i128 = fn_state.t;
        // D s_174_14: call R_set(s_174_13, s_174_12)
        let s_174_14: () = R_set(state, tracer, s_174_13, s_174_12);
        // N s_174_15: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #70u : u32
        let s_175_0: u32 = 70;
        // S s_175_1: call ConstrainUnpredictableBool(s_175_0)
        let s_175_1: bool = ConstrainUnpredictableBool(state, tracer, s_175_0);
        // D s_175_2: write-var gs#108880 <= s_175_1
        fn_state.gs_108880 = s_175_1;
        // N s_175_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
