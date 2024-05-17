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
use u_get_HCR_EL2_Type_E2H::*;
use PAR_EL1_write::*;
use CreateAccDescAT::*;
use HaveRME::*;
use CreateFaultyAddressDescriptor::*;
use AArch32_S2Translate::*;
use TTBCR_read::*;
use Mk_PAR_EL1_Type::*;
use IsExternalAbort__1::*;
use ReportAsGPCException::*;
use u_get_HCR_EL2_Type_GPF::*;
use EffectiveSCR_EL3_NSE::*;
use AArch64_S1Translate::*;
use AArch64_EncodePAR::*;
use u__UNKNOWN_bits::*;
use NoFault__1::*;
use TranslationRegime::*;
use ELUsingAArch32::*;
use AArch64_Abort::*;
use AArch64_S2Translate::*;
use EffectiveSCR_EL3_NS::*;
use SecurityStateAtEL::*;
use u_get_TTBCR_Type_EAE::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use AArch32_S1TranslateSD::*;
use AArch32_S1TranslateLD::*;
use common::*;
pub fn AArch64_AT<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    stage_in: u32,
    el_in: u8,
    ataccess: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_29268: bool,
        gs_29257: bool,
        gs_29258: bool,
        fault: ProductType1d757adad216cdef,
        ga_22720: ProductTypedc31059ca7e2391c,
        gs_29310: bool,
        regime: u32,
        addrdesc: ProductTypece7c66ccb2cab13e,
        gs_29270: bool,
        gs_29261: bool,
        gs_29309: bool,
        gs_29259: bool,
        gs_29290: bool,
        ga_22730: ProductType396b95aa74979079,
        gs_29306: bool,
        gs_29277: bool,
        ga_22724: ProductTypedc31059ca7e2391c,
        gs_29276: bool,
        el: u8,
        ga_22711: ProductType234df14d4fab6c9d,
        gs_29272: bool,
        gs_29271: bool,
        gs_29303: bool,
        gs_29304: bool,
        gs_29273: bool,
        gs_29305: bool,
        gs_29307: bool,
        is_ATS1Ex: bool,
        gs_29302: bool,
        gs_29262: bool,
        effective_nse_ns: u8,
        ga_22712: ProductTypedc31059ca7e2391c,
        gs_29267: bool,
        gs_29275: bool,
        gs_29260: bool,
        stage: u32,
        gs_29311: bool,
        ss: u32,
        accdesc: ProductType9878976b5bcce9c9,
        gs_29308: bool,
        write: bool,
        gs_29263: bool,
        ga_22709: ProductTypedc31059ca7e2391c,
        address: u64,
        stage_in: u32,
        el_in: u8,
        ataccess: u32,
    }
    let fn_state = FunctionState {
        address,
        stage_in,
        el_in,
        ataccess,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var stage_in:u32
        let s_0_0: u32 = fn_state.stage_in;
        // D s_0_1: write-var stage <= s_0_0
        fn_state.stage = s_0_0;
        // D s_0_2: read-var el_in:u8
        let s_0_2: u8 = fn_state.el_in;
        // D s_0_3: write-var el <= s_0_2
        fn_state.el = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call EffectiveSCR_EL3_NSE(s_0_4)
        let s_0_5: bool = EffectiveSCR_EL3_NSE(state, tracer, s_0_4);
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call EffectiveSCR_EL3_NS(s_0_6)
        let s_0_7: bool = EffectiveSCR_EL3_NS(state, tracer, s_0_6);
        // S s_0_8: cast zx s_0_5 -> bv
        let s_0_8: Bits = Bits::new(s_0_5 as u128, 1u16);
        // S s_0_9: cast zx s_0_7 -> bv
        let s_0_9: Bits = Bits::new(s_0_7 as u128, 1u16);
        // S s_0_10: cast reint s_0_8 -> u128
        let s_0_10: u128 = (s_0_8.value() as u128);
        // D s_0_11: size-of s_0_8
        let s_0_11: u16 = s_0_8.length();
        // S s_0_12: cast reint s_0_9 -> u128
        let s_0_12: u128 = (s_0_9.value() as u128);
        // D s_0_13: size-of s_0_9
        let s_0_13: u16 = s_0_9.length();
        // D s_0_14: lsl s_0_10 s_0_13
        let s_0_14: u128 = s_0_10 << s_0_13;
        // D s_0_15: or s_0_14 s_0_12
        let s_0_15: u128 = ((s_0_14) | (s_0_12));
        // D s_0_16: add s_0_11 s_0_13
        let s_0_16: u16 = (s_0_11 + s_0_13);
        // D s_0_17: create-bits s_0_15 s_0_16
        let s_0_17: Bits = Bits::new(s_0_15, s_0_16);
        // D s_0_18: cast reint s_0_17 -> u8
        let s_0_18: u8 = (s_0_17.value() as u8);
        // D s_0_19: write-var effective_nse_ns <= s_0_18
        fn_state.effective_nse_ns = s_0_18;
        // C s_0_20: const #() : ()
        let s_0_20: () = ();
        // S s_0_21: call HaveRME(s_0_20)
        let s_0_21: bool = HaveRME(state, tracer, s_0_20);
        // N s_0_22: branch s_0_21 b110 b1
        if s_0_21 {
            return block_110(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#29257 <= s_1_0
        fn_state.gs_29257 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#29257:u8
        let s_2_0: bool = fn_state.gs_29257;
        // N s_2_1: branch s_2_0 b109 b3
        if s_2_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#29258 <= s_3_0
        fn_state.gs_29258 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#29258:u8
        let s_4_0: bool = fn_state.gs_29258;
        // N s_4_1: branch s_4_0 b108 b5
        if s_4_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#29259 <= s_5_0
        fn_state.gs_29259 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#29259:u8
        let s_6_0: bool = fn_state.gs_29259;
        // N s_6_1: branch s_6_0 b107 b7
        if s_6_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #102552u : u32
        let s_7_0: u32 = 102552;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_HCR_EL2_Type_E2H(s_7_1)
        let s_7_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_7_1);
        // C s_7_3: const #102552u : u32
        let s_7_3: u32 = 102552;
        // D s_7_4: read-reg s_7_3:struct
        let s_7_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: call _get_HCR_EL2_Type_TGE(s_7_4)
        let s_7_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_7_4);
        // D s_7_6: cast zx s_7_2 -> bv
        let s_7_6: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 1u16);
        // D s_7_8: cast reint s_7_6 -> u128
        let s_7_8: u128 = (s_7_6.value() as u128);
        // D s_7_9: size-of s_7_6
        let s_7_9: u16 = s_7_6.length();
        // D s_7_10: cast reint s_7_7 -> u128
        let s_7_10: u128 = (s_7_7.value() as u128);
        // D s_7_11: size-of s_7_7
        let s_7_11: u16 = s_7_7.length();
        // D s_7_12: lsl s_7_8 s_7_11
        let s_7_12: u128 = s_7_8 << s_7_11;
        // D s_7_13: or s_7_12 s_7_10
        let s_7_13: u128 = ((s_7_12) | (s_7_10));
        // D s_7_14: add s_7_9 s_7_11
        let s_7_14: u16 = (s_7_9 + s_7_11);
        // D s_7_15: create-bits s_7_13 s_7_14
        let s_7_15: Bits = Bits::new(s_7_13, s_7_14);
        // D s_7_16: cast reint s_7_15 -> u8
        let s_7_16: u8 = (s_7_15.value() as u8);
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 2u16);
        // C s_7_18: const #3u : u8
        let s_7_18: u8 = 3;
        // C s_7_19: cast zx s_7_18 -> bv
        let s_7_19: Bits = Bits::new(s_7_18 as u128, 2u16);
        // D s_7_20: cmp-eq s_7_17 s_7_19
        let s_7_20: bool = ((s_7_17) == (s_7_19));
        // N s_7_21: branch s_7_20 b106 b8
        if s_7_20 {
            return block_106(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#29260 <= s_8_0
        fn_state.gs_29260 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#29260:u8
        let s_9_0: bool = fn_state.gs_29260;
        // N s_9_1: branch s_9_0 b105 b10
        if s_9_0 {
            return block_105(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#29261 <= s_10_0
        fn_state.gs_29261 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#29261:u8
        let s_11_0: bool = fn_state.gs_29261;
        // N s_11_1: branch s_11_0 b104 b12
        if s_11_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // N s_13_4: branch s_13_3 b103 b14
        if s_13_3 {
            return block_103(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#29262 <= s_14_0
        fn_state.gs_29262 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#29262:u8
        let s_15_0: bool = fn_state.gs_29262;
        // N s_15_1: branch s_15_0 b102 b16
        if s_15_0 {
            return block_102(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#29263 <= s_16_0
        fn_state.gs_29263 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#29263:u8
        let s_17_0: bool = fn_state.gs_29263;
        // N s_17_1: branch s_17_0 b101 b18
        if s_17_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var ataccess:u32
        let s_19_0: u32 = fn_state.ataccess;
        // C s_19_1: const #3u : u32
        let s_19_1: u32 = 3;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // N s_19_3: branch s_19_2 b100 b20
        if s_19_2 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var ataccess:u32
        let s_20_0: u32 = fn_state.ataccess;
        // C s_20_1: const #1u : u32
        let s_20_1: u32 = 1;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: write-var gs#29267 <= s_20_2
        fn_state.gs_29267 = s_20_2;
        // N s_20_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#29267:u8
        let s_21_0: bool = fn_state.gs_29267;
        // D s_21_1: write-var write <= s_21_0
        fn_state.write = s_21_0;
        // D s_21_2: read-var el:u8
        let s_21_2: u8 = fn_state.el;
        // D s_21_3: call SecurityStateAtEL(s_21_2)
        let s_21_3: u32 = SecurityStateAtEL(state, tracer, s_21_2);
        // D s_21_4: write-var ss <= s_21_3
        fn_state.ss = s_21_3;
        // D s_21_5: read-var ataccess:u32
        let s_21_5: u32 = fn_state.ataccess;
        // C s_21_6: const #2u : u32
        let s_21_6: u32 = 2;
        // D s_21_7: cmp-eq s_21_5 s_21_6
        let s_21_7: bool = ((s_21_5) == (s_21_6));
        // N s_21_8: branch s_21_7 b99 b22
        if s_21_7 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var ataccess:u32
        let s_22_0: u32 = fn_state.ataccess;
        // C s_22_1: const #3u : u32
        let s_22_1: u32 = 3;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: write-var gs#29268 <= s_22_2
        fn_state.gs_29268 = s_22_2;
        // N s_22_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#29268:u8
        let s_23_0: bool = fn_state.gs_29268;
        // D s_23_1: read-var ss:u32
        let s_23_1: u32 = fn_state.ss;
        // D s_23_2: read-var el:u8
        let s_23_2: u8 = fn_state.el;
        // D s_23_3: read-var write:u8
        let s_23_3: bool = fn_state.write;
        // D s_23_4: call CreateAccDescAT(s_23_1, s_23_2, s_23_3, s_23_0)
        let s_23_4: ProductType9878976b5bcce9c9 = CreateAccDescAT(
            state,
            tracer,
            s_23_1,
            s_23_2,
            s_23_3,
            s_23_0,
        );
        // D s_23_5: write-var accdesc <= s_23_4
        fn_state.accdesc = s_23_4;
        // D s_23_6: read-var accdesc:struct
        let s_23_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_23_7: call NoFault__1(s_23_6)
        let s_23_7: ProductType1d757adad216cdef = NoFault__1(state, tracer, s_23_6);
        // D s_23_8: write-var fault <= s_23_7
        fn_state.fault = s_23_7;
        // D s_23_9: read-var stage:u32
        let s_23_9: u32 = fn_state.stage;
        // C s_23_10: const #1u : u32
        let s_23_10: u32 = 1;
        // D s_23_11: cmp-eq s_23_9 s_23_10
        let s_23_11: bool = ((s_23_9) == (s_23_10));
        // N s_23_12: branch s_23_11 b98 b24
        if s_23_11 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var el:u8
        let s_24_0: u8 = fn_state.el;
        // D s_24_1: call TranslationRegime(s_24_0)
        let s_24_1: u32 = TranslationRegime(state, tracer, s_24_0);
        // D s_24_2: write-var regime <= s_24_1
        fn_state.regime = s_24_1;
        // N s_24_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var el:u8
        let s_25_0: u8 = fn_state.el;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 2u16);
        // C s_25_2: const #448u : u32
        let s_25_2: u32 = 448;
        // D s_25_3: read-reg s_25_2:u8
        let s_25_3: u8 = {
            let value = state.read_register::<u8>(s_25_2 as isize);
            tracer.read_register(s_25_2 as isize, value);
            value
        };
        // D s_25_4: cast zx s_25_3 -> bv
        let s_25_4: Bits = Bits::new(s_25_3 as u128, 2u16);
        // D s_25_5: cmp-eq s_25_1 s_25_4
        let s_25_5: bool = ((s_25_1) == (s_25_4));
        // N s_25_6: branch s_25_5 b97 b26
        if s_25_5 {
            return block_97(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#29270 <= s_26_0
        fn_state.gs_29270 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#29270:u8
        let s_27_0: bool = fn_state.gs_29270;
        // N s_27_1: branch s_27_0 b96 b28
        if s_27_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var el:u8
        let s_28_0: u8 = fn_state.el;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 2u16);
        // C s_28_2: const #448u : u32
        let s_28_2: u32 = 448;
        // D s_28_3: read-reg s_28_2:u8
        let s_28_3: u8 = {
            let value = state.read_register::<u8>(s_28_2 as isize);
            tracer.read_register(s_28_2 as isize, value);
            value
        };
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 2u16);
        // D s_28_5: cmp-ne s_28_1 s_28_4
        let s_28_5: bool = ((s_28_1) != (s_28_4));
        // N s_28_6: branch s_28_5 b95 b29
        if s_28_5 {
            return block_95(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#29271 <= s_29_0
        fn_state.gs_29271 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#29271:u8
        let s_30_0: bool = fn_state.gs_29271;
        // D s_30_1: write-var gs#29272 <= s_30_0
        fn_state.gs_29272 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#29272:u8
        let s_31_0: bool = fn_state.gs_29272;
        // N s_31_1: branch s_31_0 b89 b32
        if s_31_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var fault:struct
        let s_32_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_32_1: read-var regime:u32
        let s_32_1: u32 = fn_state.regime;
        // D s_32_2: read-var address:u64
        let s_32_2: u64 = fn_state.address;
        // C s_32_3: const #1u : u8
        let s_32_3: bool = true;
        // D s_32_4: read-var accdesc:struct
        let s_32_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_32_5: call AArch64_S1Translate(s_32_0, s_32_1, s_32_2, s_32_3, s_32_4)
        let s_32_5: ProductTypedc31059ca7e2391c = AArch64_S1Translate(
            state,
            tracer,
            s_32_0,
            s_32_1,
            s_32_2,
            s_32_3,
            s_32_4,
        );
        // D s_32_6: write-var ga#22712 <= s_32_5
        fn_state.ga_22712 = s_32_5;
        // D s_32_7: read-var ga#22712.0:struct
        let s_32_7: ProductType1d757adad216cdef = fn_state.ga_22712._0;
        // D s_32_8: read-var ga#22712.1:struct
        let s_32_8: ProductTypece7c66ccb2cab13e = fn_state.ga_22712._1;
        // D s_32_9: write-var fault <= s_32_7
        fn_state.fault = s_32_7;
        // D s_32_10: write-var addrdesc <= s_32_8
        fn_state.addrdesc = s_32_8;
        // N s_32_11: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var stage:u32
        let s_33_0: u32 = fn_state.stage;
        // C s_33_1: const #1u : u32
        let s_33_1: u32 = 1;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // N s_33_3: branch s_33_2 b88 b34
        if s_33_2 {
            return block_88(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#29273 <= s_34_0
        fn_state.gs_29273 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#29273:u8
        let s_35_0: bool = fn_state.gs_29273;
        // N s_35_1: branch s_35_0 b74 b36
        if s_35_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var stage:u32
        let s_37_0: u32 = fn_state.stage;
        // C s_37_1: const #1u : u32
        let s_37_1: u32 = 1;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: write-var is_ATS1Ex <= s_37_2
        fn_state.is_ATS1Ex = s_37_2;
        // D s_37_4: read-var fault.16:struct
        let s_37_4: u32 = fn_state.fault._16;
        // C s_37_5: const #0u : u32
        let s_37_5: u32 = 0;
        // D s_37_6: cmp-eq s_37_4 s_37_5
        let s_37_6: bool = ((s_37_4) == (s_37_5));
        // N s_37_7: branch s_37_6 b40 b38
        if s_37_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var addrdesc:struct
        let s_39_0: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_39_1: read-var regime:u32
        let s_39_1: u32 = fn_state.regime;
        // D s_39_2: read-var is_ATS1Ex:u8
        let s_39_2: bool = fn_state.is_ATS1Ex;
        // D s_39_3: call AArch64_EncodePAR(s_39_1, s_39_2, s_39_0)
        let s_39_3: () = AArch64_EncodePAR(state, tracer, s_39_1, s_39_2, s_39_0);
        // N s_39_4: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var address:u64
        let s_40_0: u64 = fn_state.address;
        // D s_40_1: read-var fault:struct
        let s_40_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_40_2: call CreateFaultyAddressDescriptor(s_40_0, s_40_1)
        let s_40_2: ProductTypece7c66ccb2cab13e = CreateFaultyAddressDescriptor(
            state,
            tracer,
            s_40_0,
            s_40_1,
        );
        // D s_40_3: write-var addrdesc <= s_40_2
        fn_state.addrdesc = s_40_2;
        // D s_40_4: read-var fault:struct
        let s_40_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_40_5: call IsExternalAbort__1(s_40_4)
        let s_40_5: bool = IsExternalAbort__1(state, tracer, s_40_4);
        // N s_40_6: branch s_40_5 b73 b41
        if s_40_5 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #16975u : u32
        let s_41_0: u32 = 16975;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: cast zx s_41_1 -> bv
        let s_41_2: Bits = Bits::new(s_41_1 as u128, 2u16);
        // C s_41_3: const #440u : u32
        let s_41_3: u32 = 440;
        // D s_41_4: read-reg s_41_3:u8
        let s_41_4: u8 = {
            let value = state.read_register::<u8>(s_41_3 as isize);
            tracer.read_register(s_41_3 as isize, value);
            value
        };
        // D s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 2u16);
        // D s_41_6: cmp-eq s_41_2 s_41_5
        let s_41_6: bool = ((s_41_2) == (s_41_5));
        // N s_41_7: branch s_41_6 b72 b42
        if s_41_6 {
            return block_72(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#29302 <= s_42_0
        fn_state.gs_29302 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#29302:u8
        let s_43_0: bool = fn_state.gs_29302;
        // D s_43_1: write-var gs#29303 <= s_43_0
        fn_state.gs_29303 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#29303:u8
        let s_44_0: bool = fn_state.gs_29303;
        // N s_44_1: branch s_44_0 b71 b45
        if s_44_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call HaveRME(s_45_0)
        let s_45_1: bool = HaveRME(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b70 b46
        if s_45_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#29304 <= s_46_0
        fn_state.gs_29304 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#29304:u8
        let s_47_0: bool = fn_state.gs_29304;
        // N s_47_1: branch s_47_0 b54 b48
        if s_47_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#29310 <= s_48_0
        fn_state.gs_29310 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#29310:u8
        let s_49_0: bool = fn_state.gs_29310;
        // D s_49_1: write-var gs#29311 <= s_49_0
        fn_state.gs_29311 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#29311:u8
        let s_50_0: bool = fn_state.gs_29311;
        // N s_50_1: branch s_50_0 b53 b51
        if s_50_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #128s : i64
        let s_53_0: i64 = 128;
        // C s_53_1: cast zx s_53_0 -> i
        let s_53_1: i128 = (i128::try_from(s_53_0).unwrap());
        // S s_53_2: call __UNKNOWN_bits(s_53_1)
        let s_53_2: Bits = u__UNKNOWN_bits(state, tracer, s_53_1);
        // S s_53_3: cast reint s_53_2 -> u128
        let s_53_3: u128 = (s_53_2.value() as u128);
        // S s_53_4: call Mk_PAR_EL1_Type(s_53_3)
        let s_53_4: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(state, tracer, s_53_3);
        // S s_53_5: call PAR_EL1_write(s_53_4)
        let s_53_5: () = PAR_EL1_write(state, tracer, s_53_4);
        // D s_53_6: read-var addrdesc.0:struct
        let s_53_6: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_53_7: read-var address:u64
        let s_53_7: u64 = fn_state.address;
        // D s_53_8: call AArch64_Abort(s_53_7, s_53_6)
        let s_53_8: () = AArch64_Abort(state, tracer, s_53_7, s_53_6);
        // N s_53_9: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var fault:struct
        let s_54_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_54_1: call ReportAsGPCException(s_54_0)
        let s_54_1: bool = ReportAsGPCException(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b69 b55
        if s_54_1 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #102552u : u32
        let s_55_0: u32 = 102552;
        // D s_55_1: read-reg s_55_0:struct
        let s_55_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call _get_HCR_EL2_Type_GPF(s_55_1)
        let s_55_2: bool = u_get_HCR_EL2_Type_GPF(state, tracer, s_55_1);
        // D s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #1u : u8
        let s_55_4: bool = true;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // D s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // N s_55_7: branch s_55_6 b68 b56
        if s_55_6 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#29305 <= s_56_0
        fn_state.gs_29305 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#29305:u8
        let s_57_0: bool = fn_state.gs_29305;
        // N s_57_1: branch s_57_0 b64 b58
        if s_57_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#29307 <= s_58_0
        fn_state.gs_29307 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#29307:u8
        let s_59_0: bool = fn_state.gs_29307;
        // N s_59_1: branch s_59_0 b63 b60
        if s_59_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#29308 <= s_60_0
        fn_state.gs_29308 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#29308:u8
        let s_61_0: bool = fn_state.gs_29308;
        // D s_61_1: write-var gs#29309 <= s_61_0
        fn_state.gs_29309 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#29309:u8
        let s_62_0: bool = fn_state.gs_29309;
        // D s_62_1: write-var gs#29310 <= s_62_0
        fn_state.gs_29310 = s_62_0;
        // N s_62_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var is_ATS1Ex:u8
        let s_63_0: bool = fn_state.is_ATS1Ex;
        // D s_63_1: write-var gs#29308 <= s_63_0
        fn_state.gs_29308 = s_63_0;
        // N s_63_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var el:u8
        let s_64_0: u8 = fn_state.el;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 2u16);
        // C s_64_2: const #440u : u32
        let s_64_2: u32 = 440;
        // D s_64_3: read-reg s_64_2:u8
        let s_64_3: u8 = {
            let value = state.read_register::<u8>(s_64_2 as isize);
            tracer.read_register(s_64_2 as isize, value);
            value
        };
        // D s_64_4: cast zx s_64_3 -> bv
        let s_64_4: Bits = Bits::new(s_64_3 as u128, 2u16);
        // D s_64_5: cmp-eq s_64_1 s_64_4
        let s_64_5: bool = ((s_64_1) == (s_64_4));
        // N s_64_6: branch s_64_5 b67 b65
        if s_64_5 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var el:u8
        let s_65_0: u8 = fn_state.el;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 2u16);
        // C s_65_2: const #448u : u32
        let s_65_2: u32 = 448;
        // D s_65_3: read-reg s_65_2:u8
        let s_65_3: u8 = {
            let value = state.read_register::<u8>(s_65_2 as isize);
            tracer.read_register(s_65_2 as isize, value);
            value
        };
        // D s_65_4: cast zx s_65_3 -> bv
        let s_65_4: Bits = Bits::new(s_65_3 as u128, 2u16);
        // D s_65_5: cmp-eq s_65_1 s_65_4
        let s_65_5: bool = ((s_65_1) == (s_65_4));
        // D s_65_6: write-var gs#29306 <= s_65_5
        fn_state.gs_29306 = s_65_5;
        // N s_65_7: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#29306:u8
        let s_66_0: bool = fn_state.gs_29306;
        // D s_66_1: write-var gs#29307 <= s_66_0
        fn_state.gs_29307 = s_66_0;
        // N s_66_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#29306 <= s_67_0
        fn_state.gs_29306 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #16975u : u32
        let s_68_0: u32 = 16975;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: cast zx s_68_1 -> bv
        let s_68_2: Bits = Bits::new(s_68_1 as u128, 2u16);
        // C s_68_3: const #440u : u32
        let s_68_3: u32 = 440;
        // D s_68_4: read-reg s_68_3:u8
        let s_68_4: u8 = {
            let value = state.read_register::<u8>(s_68_3 as isize);
            tracer.read_register(s_68_3 as isize, value);
            value
        };
        // D s_68_5: cast zx s_68_4 -> bv
        let s_68_5: Bits = Bits::new(s_68_4 as u128, 2u16);
        // D s_68_6: cmp-eq s_68_2 s_68_5
        let s_68_6: bool = ((s_68_2) == (s_68_5));
        // D s_68_7: write-var gs#29305 <= s_68_6
        fn_state.gs_29305 = s_68_6;
        // N s_68_8: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#29309 <= s_69_0
        fn_state.gs_29309 = s_69_0;
        // N s_69_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var fault.6:struct
        let s_70_0: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_70_1: write-var ga#22730 <= s_70_0
        fn_state.ga_22730 = s_70_0;
        // D s_70_2: read-var ga#22730.0:struct
        let s_70_2: u32 = fn_state.ga_22730._0;
        // C s_70_3: const #0u : u32
        let s_70_3: u32 = 0;
        // D s_70_4: cmp-eq s_70_2 s_70_3
        let s_70_4: bool = ((s_70_2) == (s_70_3));
        // D s_70_5: write-var gs#29304 <= s_70_4
        fn_state.gs_29304 = s_70_4;
        // N s_70_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#29311 <= s_71_0
        fn_state.gs_29311 = s_71_0;
        // N s_71_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var fault.14:struct
        let s_72_0: bool = fn_state.fault._14;
        // D s_72_1: write-var gs#29302 <= s_72_0
        fn_state.gs_29302 = s_72_0;
        // N s_72_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#29303 <= s_73_0
        fn_state.gs_29303 = s_73_0;
        // N s_73_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #440u : u32
        let s_74_0: u32 = 440;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call ELUsingAArch32(s_74_1)
        let s_74_2: bool = ELUsingAArch32(state, tracer, s_74_1);
        // N s_74_3: branch s_74_2 b87 b75
        if s_74_2 {
            return block_87(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#29275 <= s_75_0
        fn_state.gs_29275 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#29275:u8
        let s_76_0: bool = fn_state.gs_29275;
        // N s_76_1: branch s_76_0 b86 b77
        if s_76_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#29276 <= s_77_0
        fn_state.gs_29276 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#29276:u8
        let s_78_0: bool = fn_state.gs_29276;
        // N s_78_1: branch s_78_0 b85 b79
        if s_78_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var regime:u32
        let s_79_0: u32 = fn_state.regime;
        // C s_79_1: const #4u : u32
        let s_79_1: u32 = 4;
        // D s_79_2: cmp-eq s_79_0 s_79_1
        let s_79_2: bool = ((s_79_0) == (s_79_1));
        // N s_79_3: branch s_79_2 b84 b80
        if s_79_2 {
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
        // D s_80_1: write-var gs#29277 <= s_80_0
        fn_state.gs_29277 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#29277:u8
        let s_81_0: bool = fn_state.gs_29277;
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
        // N s_82_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // C s_83_1: const #() : ()
        let s_83_1: () = ();
        // D s_83_2: create-sum enum = 0:"s_83_1"
        let s_83_2: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_0(s_83_1);
        // D s_83_3: read-var fault:struct
        let s_83_3: ProductType1d757adad216cdef = fn_state.fault;
        // D s_83_4: read-var addrdesc:struct
        let s_83_4: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // C s_83_5: const #1u : u8
        let s_83_5: bool = true;
        // D s_83_6: read-var accdesc:struct
        let s_83_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_83_7: call AArch64_S2Translate(s_83_3, s_83_4, s_83_0, s_83_2, s_83_5, s_83_6)
        let s_83_7: ProductTypedc31059ca7e2391c = AArch64_S2Translate(
            state,
            tracer,
            s_83_3,
            s_83_4,
            s_83_0,
            s_83_2,
            s_83_5,
            s_83_6,
        );
        // D s_83_8: write-var ga#22724 <= s_83_7
        fn_state.ga_22724 = s_83_7;
        // D s_83_9: read-var ga#22724.0:struct
        let s_83_9: ProductType1d757adad216cdef = fn_state.ga_22724._0;
        // D s_83_10: read-var ga#22724.1:struct
        let s_83_10: ProductTypece7c66ccb2cab13e = fn_state.ga_22724._1;
        // D s_83_11: write-var fault <= s_83_9
        fn_state.fault = s_83_9;
        // D s_83_12: write-var addrdesc <= s_83_10
        fn_state.addrdesc = s_83_10;
        // N s_83_13: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call EL2Enabled(s_84_0)
        let s_84_1: bool = EL2Enabled(state, tracer, s_84_0);
        // D s_84_2: write-var gs#29277 <= s_84_1
        fn_state.gs_29277 = s_84_1;
        // N s_84_3: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #64s : i
        let s_85_0: i128 = 64;
        // D s_85_1: read-var address:u64
        let s_85_1: u64 = fn_state.address;
        // D s_85_2: cast zx s_85_1 -> bv
        let s_85_2: Bits = Bits::new(s_85_1 as u128, 64u16);
        // D s_85_3: bits-cast zx s_85_2 -> bv length s_85_0
        let s_85_3: Bits = s_85_2.zero_extend(s_85_0);
        // D s_85_4: cast reint s_85_3 -> u64
        let s_85_4: u64 = (s_85_3.value() as u64);
        // D s_85_5: write-var addrdesc.7 <= s_85_4
        fn_state.addrdesc._7 = s_85_4;
        // C s_85_6: const #() : ()
        let s_85_6: () = ();
        // D s_85_7: create-sum enum = 0:"s_85_6"
        let s_85_7: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_0(s_85_6);
        // D s_85_8: read-var fault:struct
        let s_85_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_85_9: read-var addrdesc:struct
        let s_85_9: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // C s_85_10: const #1u : u8
        let s_85_10: bool = true;
        // D s_85_11: read-var accdesc:struct
        let s_85_11: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_85_12: call AArch32_S2Translate(s_85_8, s_85_9, s_85_7, s_85_10, s_85_11)
        let s_85_12: ProductTypedc31059ca7e2391c = AArch32_S2Translate(
            state,
            tracer,
            s_85_8,
            s_85_9,
            s_85_7,
            s_85_10,
            s_85_11,
        );
        // D s_85_13: write-var ga#22720 <= s_85_12
        fn_state.ga_22720 = s_85_12;
        // D s_85_14: read-var ga#22720.0:struct
        let s_85_14: ProductType1d757adad216cdef = fn_state.ga_22720._0;
        // D s_85_15: read-var ga#22720.1:struct
        let s_85_15: ProductTypece7c66ccb2cab13e = fn_state.ga_22720._1;
        // D s_85_16: write-var fault <= s_85_14
        fn_state.fault = s_85_14;
        // D s_85_17: write-var addrdesc <= s_85_15
        fn_state.addrdesc = s_85_15;
        // N s_85_18: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call EL2Enabled(s_86_0)
        let s_86_1: bool = EL2Enabled(state, tracer, s_86_0);
        // D s_86_2: write-var gs#29276 <= s_86_1
        fn_state.gs_29276 = s_86_1;
        // N s_86_3: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var regime:u32
        let s_87_0: u32 = fn_state.regime;
        // C s_87_1: const #4u : u32
        let s_87_1: u32 = 4;
        // D s_87_2: cmp-eq s_87_0 s_87_1
        let s_87_2: bool = ((s_87_0) == (s_87_1));
        // D s_87_3: write-var gs#29275 <= s_87_2
        fn_state.gs_29275 = s_87_2;
        // N s_87_4: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var fault.16:struct
        let s_88_0: u32 = fn_state.fault._16;
        // C s_88_1: const #0u : u32
        let s_88_1: u32 = 0;
        // D s_88_2: cmp-eq s_88_0 s_88_1
        let s_88_2: bool = ((s_88_0) == (s_88_1));
        // D s_88_3: write-var gs#29273 <= s_88_2
        fn_state.gs_29273 = s_88_2;
        // N s_88_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var regime:u32
        let s_89_0: u32 = fn_state.regime;
        // C s_89_1: const #2u : u32
        let s_89_1: u32 = 2;
        // D s_89_2: cmp-eq s_89_0 s_89_1
        let s_89_2: bool = ((s_89_0) == (s_89_1));
        // N s_89_3: branch s_89_2 b94 b90
        if s_89_2 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #() : ()
        let s_90_0: () = ();
        // S s_90_1: call TTBCR_read(s_90_0)
        let s_90_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_90_0);
        // S s_90_2: call _get_TTBCR_Type_EAE(s_90_1)
        let s_90_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_90_1);
        // S s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // C s_90_4: const #1u : u8
        let s_90_4: bool = true;
        // C s_90_5: cast zx s_90_4 -> bv
        let s_90_5: Bits = Bits::new(s_90_4 as u128, 1u16);
        // S s_90_6: cmp-eq s_90_3 s_90_5
        let s_90_6: bool = ((s_90_3) == (s_90_5));
        // D s_90_7: write-var gs#29290 <= s_90_6
        fn_state.gs_29290 = s_90_6;
        // N s_90_8: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#29290:u8
        let s_91_0: bool = fn_state.gs_29290;
        // N s_91_1: branch s_91_0 b93 b92
        if s_91_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0s : i
        let s_92_0: i128 = 0;
        // D s_92_1: read-var address:u64
        let s_92_1: u64 = fn_state.address;
        // D s_92_2: cast zx s_92_1 -> bv
        let s_92_2: Bits = Bits::new(s_92_1 as u128, 64u16);
        // C s_92_3: const #1s : i64
        let s_92_3: i64 = 1;
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // C s_92_5: const #31s : i
        let s_92_5: i128 = 31;
        // C s_92_6: add s_92_5 s_92_4
        let s_92_6: i128 = (s_92_5 + s_92_4);
        // D s_92_7: bit-extract s_92_2 s_92_0 s_92_6
        let s_92_7: Bits = (Bits::new(
            ((s_92_2) >> (s_92_0)).value(),
            u16::try_from(s_92_6).unwrap(),
        ));
        // D s_92_8: cast reint s_92_7 -> u32
        let s_92_8: u32 = (s_92_7.value() as u32);
        // D s_92_9: read-var fault:struct
        let s_92_9: ProductType1d757adad216cdef = fn_state.fault;
        // D s_92_10: read-var regime:u32
        let s_92_10: u32 = fn_state.regime;
        // C s_92_11: const #1u : u8
        let s_92_11: bool = true;
        // D s_92_12: read-var accdesc:struct
        let s_92_12: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_92_13: call AArch32_S1TranslateSD(s_92_9, s_92_10, s_92_8, s_92_11, s_92_12)
        let s_92_13: ProductType234df14d4fab6c9d = AArch32_S1TranslateSD(
            state,
            tracer,
            s_92_9,
            s_92_10,
            s_92_8,
            s_92_11,
            s_92_12,
        );
        // D s_92_14: write-var ga#22711 <= s_92_13
        fn_state.ga_22711 = s_92_13;
        // D s_92_15: read-var ga#22711.0:struct
        let s_92_15: ProductType1d757adad216cdef = fn_state.ga_22711._0;
        // D s_92_16: read-var ga#22711.1:struct
        let s_92_16: ProductTypece7c66ccb2cab13e = fn_state.ga_22711._1;
        // D s_92_17: write-var fault <= s_92_15
        fn_state.fault = s_92_15;
        // D s_92_18: write-var addrdesc <= s_92_16
        fn_state.addrdesc = s_92_16;
        // N s_92_19: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0s : i
        let s_93_0: i128 = 0;
        // D s_93_1: read-var address:u64
        let s_93_1: u64 = fn_state.address;
        // D s_93_2: cast zx s_93_1 -> bv
        let s_93_2: Bits = Bits::new(s_93_1 as u128, 64u16);
        // C s_93_3: const #1s : i64
        let s_93_3: i64 = 1;
        // C s_93_4: cast zx s_93_3 -> i
        let s_93_4: i128 = (i128::try_from(s_93_3).unwrap());
        // C s_93_5: const #31s : i
        let s_93_5: i128 = 31;
        // C s_93_6: add s_93_5 s_93_4
        let s_93_6: i128 = (s_93_5 + s_93_4);
        // D s_93_7: bit-extract s_93_2 s_93_0 s_93_6
        let s_93_7: Bits = (Bits::new(
            ((s_93_2) >> (s_93_0)).value(),
            u16::try_from(s_93_6).unwrap(),
        ));
        // D s_93_8: cast reint s_93_7 -> u32
        let s_93_8: u32 = (s_93_7.value() as u32);
        // D s_93_9: read-var fault:struct
        let s_93_9: ProductType1d757adad216cdef = fn_state.fault;
        // D s_93_10: read-var regime:u32
        let s_93_10: u32 = fn_state.regime;
        // C s_93_11: const #1u : u8
        let s_93_11: bool = true;
        // D s_93_12: read-var accdesc:struct
        let s_93_12: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_93_13: call AArch32_S1TranslateLD(s_93_9, s_93_10, s_93_8, s_93_11, s_93_12)
        let s_93_13: ProductTypedc31059ca7e2391c = AArch32_S1TranslateLD(
            state,
            tracer,
            s_93_9,
            s_93_10,
            s_93_8,
            s_93_11,
            s_93_12,
        );
        // D s_93_14: write-var ga#22709 <= s_93_13
        fn_state.ga_22709 = s_93_13;
        // D s_93_15: read-var ga#22709.0:struct
        let s_93_15: ProductType1d757adad216cdef = fn_state.ga_22709._0;
        // D s_93_16: read-var ga#22709.1:struct
        let s_93_16: ProductTypece7c66ccb2cab13e = fn_state.ga_22709._1;
        // D s_93_17: write-var fault <= s_93_15
        fn_state.fault = s_93_15;
        // D s_93_18: write-var addrdesc <= s_93_16
        fn_state.addrdesc = s_93_16;
        // N s_93_19: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var gs#29290 <= s_94_0
        fn_state.gs_29290 = s_94_0;
        // N s_94_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var el:u8
        let s_95_0: u8 = fn_state.el;
        // D s_95_1: call ELUsingAArch32(s_95_0)
        let s_95_1: bool = ELUsingAArch32(state, tracer, s_95_0);
        // D s_95_2: write-var gs#29271 <= s_95_1
        fn_state.gs_29271 = s_95_1;
        // N s_95_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #1u : u8
        let s_96_0: bool = true;
        // D s_96_1: write-var gs#29272 <= s_96_0
        fn_state.gs_29272 = s_96_0;
        // N s_96_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #440u : u32
        let s_97_0: u32 = 440;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call ELUsingAArch32(s_97_1)
        let s_97_2: bool = ELUsingAArch32(state, tracer, s_97_1);
        // D s_97_3: write-var gs#29270 <= s_97_2
        fn_state.gs_29270 = s_97_2;
        // N s_97_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #4u : u32
        let s_98_0: u32 = 4;
        // D s_98_1: write-var regime <= s_98_0
        fn_state.regime = s_98_0;
        // N s_98_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #1u : u8
        let s_99_0: bool = true;
        // D s_99_1: write-var gs#29268 <= s_99_0
        fn_state.gs_29268 = s_99_0;
        // N s_99_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #1u : u8
        let s_100_0: bool = true;
        // D s_100_1: write-var gs#29267 <= s_100_0
        fn_state.gs_29267 = s_100_0;
        // N s_100_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u32
        let s_101_0: u32 = 0;
        // D s_101_1: write-var stage <= s_101_0
        fn_state.stage = s_101_0;
        // N s_101_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #() : ()
        let s_102_0: () = ();
        // S s_102_1: call EL2Enabled(s_102_0)
        let s_102_1: bool = EL2Enabled(state, tracer, s_102_0);
        // S s_102_2: not s_102_1
        let s_102_2: bool = !s_102_1;
        // D s_102_3: write-var gs#29263 <= s_102_2
        fn_state.gs_29263 = s_102_2;
        // N s_102_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var stage:u32
        let s_103_0: u32 = fn_state.stage;
        // C s_103_1: const #1u : u32
        let s_103_1: u32 = 1;
        // D s_103_2: cmp-eq s_103_0 s_103_1
        let s_103_2: bool = ((s_103_0) == (s_103_1));
        // D s_103_3: write-var gs#29262 <= s_103_2
        fn_state.gs_29262 = s_103_2;
        // N s_103_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #432u : u32
        let s_104_0: u32 = 432;
        // D s_104_1: read-reg s_104_0:u8
        let s_104_1: u8 = {
            let value = state.read_register::<u8>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: write-var el <= s_104_1
        fn_state.el = s_104_1;
        // N s_104_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var stage:u32
        let s_105_0: u32 = fn_state.stage;
        // C s_105_1: const #0u : u32
        let s_105_1: u32 = 0;
        // D s_105_2: cmp-eq s_105_0 s_105_1
        let s_105_2: bool = ((s_105_0) == (s_105_1));
        // D s_105_3: write-var gs#29261 <= s_105_2
        fn_state.gs_29261 = s_105_2;
        // N s_105_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var el:u8
        let s_106_0: u8 = fn_state.el;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 2u16);
        // C s_106_2: const #440u : u32
        let s_106_2: u32 = 440;
        // D s_106_3: read-reg s_106_2:u8
        let s_106_3: u8 = {
            let value = state.read_register::<u8>(s_106_2 as isize);
            tracer.read_register(s_106_2 as isize, value);
            value
        };
        // D s_106_4: cast zx s_106_3 -> bv
        let s_106_4: Bits = Bits::new(s_106_3 as u128, 2u16);
        // D s_106_5: cmp-eq s_106_1 s_106_4
        let s_106_5: bool = ((s_106_1) == (s_106_4));
        // D s_106_6: write-var gs#29260 <= s_106_5
        fn_state.gs_29260 = s_106_5;
        // N s_106_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_107_0: panic
        panic!("{:?}", ());
        // N s_107_1: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var el:u8
        let s_108_0: u8 = fn_state.el;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 2u16);
        // C s_108_2: const #424u : u32
        let s_108_2: u32 = 424;
        // D s_108_3: read-reg s_108_2:u8
        let s_108_3: u8 = {
            let value = state.read_register::<u8>(s_108_2 as isize);
            tracer.read_register(s_108_2 as isize, value);
            value
        };
        // D s_108_4: cast zx s_108_3 -> bv
        let s_108_4: Bits = Bits::new(s_108_3 as u128, 2u16);
        // D s_108_5: cmp-ne s_108_1 s_108_4
        let s_108_5: bool = ((s_108_1) != (s_108_4));
        // D s_108_6: write-var gs#29259 <= s_108_5
        fn_state.gs_29259 = s_108_5;
        // N s_108_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var effective_nse_ns:u8
        let s_109_0: u8 = fn_state.effective_nse_ns;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 2u16);
        // C s_109_2: const #2u : u8
        let s_109_2: u8 = 2;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 2u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#29258 <= s_109_4
        fn_state.gs_29258 = s_109_4;
        // N s_109_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #16975u : u32
        let s_110_0: u32 = 16975;
        // D s_110_1: read-reg s_110_0:u8
        let s_110_1: u8 = {
            let value = state.read_register::<u8>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // D s_110_2: cast zx s_110_1 -> bv
        let s_110_2: Bits = Bits::new(s_110_1 as u128, 2u16);
        // C s_110_3: const #424u : u32
        let s_110_3: u32 = 424;
        // D s_110_4: read-reg s_110_3:u8
        let s_110_4: u8 = {
            let value = state.read_register::<u8>(s_110_3 as isize);
            tracer.read_register(s_110_3 as isize, value);
            value
        };
        // D s_110_5: cast zx s_110_4 -> bv
        let s_110_5: Bits = Bits::new(s_110_4 as u128, 2u16);
        // D s_110_6: cmp-eq s_110_2 s_110_5
        let s_110_6: bool = ((s_110_2) == (s_110_5));
        // D s_110_7: write-var gs#29257 <= s_110_6
        fn_state.gs_29257 = s_110_6;
        // N s_110_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
