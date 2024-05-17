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
use IsSCTLR2EL2Enabled::*;
use HaveDoubleFault2Ext::*;
use IsInHost::*;
use HaveDoubleFaultExt::*;
use u_get_HCRX_EL2_Type_TMEA::*;
use IsSCTLR2EL1Enabled::*;
use u_get_SCTLR2_EL2_Type_NMEA::*;
use u__UNKNOWN_bits::*;
use u_get_SCR_EL3_Type_TMEA::*;
use IsHCRXEL2Enabled::*;
use u_get_SCR_EL3_Type_NMEA::*;
use u_get_SCTLR2_EL1_Type_NMEA::*;
use EffectiveEA::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_AMO::*;
use common::*;
pub fn AArch64_PhysicalSErrorTarget<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_24255: (),
) -> ProductTypea5cc8de4daab131c {
    #[derive(Default)]
    struct FunctionState {
        gs_24311: bool,
        gs_24310: bool,
        nmea_bit: bool,
        gs_24293: bool,
        ga_18911: u8,
        gs_24317: bool,
        gs_24271: bool,
        gs_24309: bool,
        gs_24268: bool,
        gs_24320: bool,
        gs_24319: bool,
        gs_24291: bool,
        gs_24304: bool,
        gs_24314: bool,
        take_in_el3: bool,
        gs_24302: bool,
        gs_24294: bool,
        masked: bool,
        gs_24297: bool,
        route_masked_to_el3: bool,
        gs_24307: bool,
        gs_24298: bool,
        gs_24263: bool,
        gs_24279: bool,
        route_to_el3: bool,
        gs_24295: bool,
        gs_24287: bool,
        gs_24323: bool,
        gs_24265: bool,
        gs_24308: bool,
        gs_24326: bool,
        u_1261: bool,
        gs_24296: bool,
        gs_24274: bool,
        gs_24305: bool,
        gs_24273: bool,
        gs_24270: bool,
        route_to_el2: bool,
        take_in_el1_0: bool,
        ga_18894: u8,
        gs_24313: bool,
        gs_24312: bool,
        gs_24257: bool,
        take_in_el2_0: bool,
        gs_24321: bool,
        gs_24315: bool,
        gs_24292: bool,
        gs_24299: bool,
        route_masked_to_el2: bool,
        gs_24306: bool,
        gs_24325: bool,
        gs_24256: bool,
        target_el: u8,
        gs_24316: bool,
        gs_24267: bool,
        gs_24261: bool,
        gs_24301: bool,
        gs_24318: bool,
        gs_24260: bool,
        gs_24303: bool,
        gs_24278: bool,
        gs_24259: bool,
        gs_24322: bool,
        gs_24255: (),
    }
    let fn_state = FunctionState {
        gs_24255,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-ne s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) != (s_0_5));
        // N s_0_7: branch s_0_6 b202 b1
        if s_0_6 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var route_to_el3 <= s_1_0
        fn_state.route_to_el3 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_2_0: read-var route_to_el3:u8
        let s_2_0: bool = fn_state.route_to_el3;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b201 b3
        if s_2_1 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#24256 <= s_3_0
        fn_state.gs_24256 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_4_0: read-var gs#24256:u8
        let s_4_0: bool = fn_state.gs_24256;
        // N s_4_1: branch s_4_0 b200 b5
        if s_4_0 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#24257 <= s_5_0
        fn_state.gs_24257 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_6_0: read-var gs#24257:u8
        let s_6_0: bool = fn_state.gs_24257;
        // N s_6_1: branch s_6_0 b199 b7
        if s_6_0 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_7_0: read-var route_to_el3:u8
        let s_7_0: bool = fn_state.route_to_el3;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // N s_7_2: branch s_7_1 b198 b8
        if s_7_1 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#24259 <= s_8_0
        fn_state.gs_24259 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_9_0: read-var gs#24259:u8
        let s_9_0: bool = fn_state.gs_24259;
        // N s_9_1: branch s_9_0 b197 b10
        if s_9_0 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#24260 <= s_10_0
        fn_state.gs_24260 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_11_0: read-var gs#24260:u8
        let s_11_0: bool = fn_state.gs_24260;
        // N s_11_1: branch s_11_0 b193 b12
        if s_11_0 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var route_to_el2 <= s_12_0
        fn_state.route_to_el2 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_13_0: const #16975u : u32
        let s_13_0: u32 = 16975;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: write-var ga#18894 <= s_13_1
        fn_state.ga_18894 = s_13_1;
        // D s_13_3: read-var ga#18894:u8
        let s_13_3: u8 = fn_state.ga_18894;
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 2u16);
        // C s_13_5: const #424u : u32
        let s_13_5: u32 = 424;
        // D s_13_6: read-reg s_13_5:u8
        let s_13_6: u8 = {
            let value = state.read_register::<u8>(s_13_5 as isize);
            tracer.read_register(s_13_5 as isize, value);
            value
        };
        // D s_13_7: cast zx s_13_6 -> bv
        let s_13_7: Bits = Bits::new(s_13_6 as u128, 2u16);
        // D s_13_8: cmp-eq s_13_4 s_13_7
        let s_13_8: bool = ((s_13_4) == (s_13_7));
        // D s_13_9: not s_13_8
        let s_13_9: bool = !s_13_8;
        // N s_13_10: branch s_13_9 b168 b14
        if s_13_9 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EffectiveEA(s_14_0)
        let s_14_1: bool = EffectiveEA(state, tracer, s_14_0);
        // S s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 1u16);
        // C s_14_3: const #0u : u8
        let s_14_3: bool = false;
        // C s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 1u16);
        // S s_14_5: cmp-eq s_14_2 s_14_4
        let s_14_5: bool = ((s_14_2) == (s_14_4));
        // N s_14_6: branch s_14_5 b167 b15
        if s_14_5 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_15_0: const #16968u : u32
        let s_15_0: u32 = 16968;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: bool = {
            let value = state.read_register::<bool>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 1u16);
        // C s_15_3: const #1u : u8
        let s_15_3: bool = true;
        // C s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 1u16);
        // D s_15_5: cmp-eq s_15_2 s_15_4
        let s_15_5: bool = ((s_15_2) == (s_15_4));
        // D s_15_6: write-var gs#24265 <= s_15_5
        fn_state.gs_24265 = s_15_5;
        // N s_15_7: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_16_0: read-var gs#24265:u8
        let s_16_0: bool = fn_state.gs_24265;
        // D s_16_1: write-var masked <= s_16_0
        fn_state.masked = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HaveDoubleFault2Ext(s_17_0)
        let s_17_1: bool = HaveDoubleFault2Ext(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b140 b18
        if s_17_1 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaveDoubleFaultExt(s_18_0)
        let s_18_1: bool = HaveDoubleFaultExt(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b139 b19
        if s_18_1 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#24278 <= s_19_0
        fn_state.gs_24278 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_20_0: read-var gs#24278:u8
        let s_20_0: bool = fn_state.gs_24278;
        // N s_20_1: branch s_20_0 b135 b21
        if s_20_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HaveDoubleFault2Ext(s_22_0)
        let s_22_1: bool = HaveDoubleFault2Ext(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b80 b23
        if s_22_1 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var route_masked_to_el2 <= s_23_0
        fn_state.route_masked_to_el2 = s_23_0;
        // C s_23_2: const #0u : u8
        let s_23_2: bool = false;
        // D s_23_3: write-var route_masked_to_el3 <= s_23_2
        fn_state.route_masked_to_el3 = s_23_2;
        // N s_23_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_24_0: const #16975u : u32
        let s_24_0: u32 = 16975;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 2u16);
        // C s_24_3: const #424u : u32
        let s_24_3: u32 = 424;
        // D s_24_4: read-reg s_24_3:u8
        let s_24_4: u8 = {
            let value = state.read_register::<u8>(s_24_3 as isize);
            tracer.read_register(s_24_3 as isize, value);
            value
        };
        // D s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 2u16);
        // D s_24_6: cmp-eq s_24_2 s_24_5
        let s_24_6: bool = ((s_24_2) == (s_24_5));
        // N s_24_7: branch s_24_6 b79 b25
        if s_24_6 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#24310 <= s_25_0
        fn_state.gs_24310 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_26_0: read-var gs#24310:u8
        let s_26_0: bool = fn_state.gs_24310;
        // D s_26_1: write-var take_in_el3 <= s_26_0
        fn_state.take_in_el3 = s_26_0;
        // C s_26_2: const #16975u : u32
        let s_26_2: u32 = 16975;
        // D s_26_3: read-reg s_26_2:u8
        let s_26_3: u8 = {
            let value = state.read_register::<u8>(s_26_2 as isize);
            tracer.read_register(s_26_2 as isize, value);
            value
        };
        // D s_26_4: cast zx s_26_3 -> bv
        let s_26_4: Bits = Bits::new(s_26_3 as u128, 2u16);
        // C s_26_5: const #432u : u32
        let s_26_5: u32 = 432;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: cast zx s_26_6 -> bv
        let s_26_7: Bits = Bits::new(s_26_6 as u128, 2u16);
        // D s_26_8: cmp-eq s_26_4 s_26_7
        let s_26_8: bool = ((s_26_4) == (s_26_7));
        // N s_26_9: branch s_26_8 b78 b27
        if s_26_8 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call IsInHost(s_27_0)
        let s_27_1: bool = IsInHost(state, tracer, s_27_0);
        // D s_27_2: write-var gs#24311 <= s_27_1
        fn_state.gs_24311 = s_27_1;
        // N s_27_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_28_0: read-var gs#24311:u8
        let s_28_0: bool = fn_state.gs_24311;
        // N s_28_1: branch s_28_0 b74 b29
        if s_28_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#24313 <= s_29_0
        fn_state.gs_24313 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_30_0: read-var gs#24313:u8
        let s_30_0: bool = fn_state.gs_24313;
        // N s_30_1: branch s_30_0 b73 b31
        if s_30_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#24314 <= s_31_0
        fn_state.gs_24314 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_32_0: read-var gs#24314:u8
        let s_32_0: bool = fn_state.gs_24314;
        // D s_32_1: write-var take_in_el2_0 <= s_32_0
        fn_state.take_in_el2_0 = s_32_0;
        // C s_32_2: const #16975u : u32
        let s_32_2: u32 = 16975;
        // D s_32_3: read-reg s_32_2:u8
        let s_32_3: u8 = {
            let value = state.read_register::<u8>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // D s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 2u16);
        // C s_32_5: const #440u : u32
        let s_32_5: u32 = 440;
        // D s_32_6: read-reg s_32_5:u8
        let s_32_6: u8 = {
            let value = state.read_register::<u8>(s_32_5 as isize);
            tracer.read_register(s_32_5 as isize, value);
            value
        };
        // D s_32_7: cast zx s_32_6 -> bv
        let s_32_7: Bits = Bits::new(s_32_6 as u128, 2u16);
        // D s_32_8: cmp-eq s_32_4 s_32_7
        let s_32_8: bool = ((s_32_4) == (s_32_7));
        // N s_32_9: branch s_32_8 b72 b33
        if s_32_8 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_33_0: const #16975u : u32
        let s_33_0: u32 = 16975;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: cast zx s_33_1 -> bv
        let s_33_2: Bits = Bits::new(s_33_1 as u128, 2u16);
        // C s_33_3: const #448u : u32
        let s_33_3: u32 = 448;
        // D s_33_4: read-reg s_33_3:u8
        let s_33_4: u8 = {
            let value = state.read_register::<u8>(s_33_3 as isize);
            tracer.read_register(s_33_3 as isize, value);
            value
        };
        // D s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 2u16);
        // D s_33_6: cmp-eq s_33_2 s_33_5
        let s_33_6: bool = ((s_33_2) == (s_33_5));
        // N s_33_7: branch s_33_6 b71 b34
        if s_33_6 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#24315 <= s_34_0
        fn_state.gs_24315 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_35_0: read-var gs#24315:u8
        let s_35_0: bool = fn_state.gs_24315;
        // D s_35_1: write-var gs#24316 <= s_35_0
        fn_state.gs_24316 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_36_0: read-var gs#24316:u8
        let s_36_0: bool = fn_state.gs_24316;
        // N s_36_1: branch s_36_0 b67 b37
        if s_36_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#24318 <= s_37_0
        fn_state.gs_24318 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_38_0: read-var gs#24318:u8
        let s_38_0: bool = fn_state.gs_24318;
        // N s_38_1: branch s_38_0 b63 b39
        if s_38_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#24320 <= s_39_0
        fn_state.gs_24320 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_40_0: read-var gs#24320:u8
        let s_40_0: bool = fn_state.gs_24320;
        // N s_40_1: branch s_40_0 b62 b41
        if s_40_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#24321 <= s_41_0
        fn_state.gs_24321 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_42_0: read-var gs#24321:u8
        let s_42_0: bool = fn_state.gs_24321;
        // D s_42_1: write-var take_in_el1_0 <= s_42_0
        fn_state.take_in_el1_0 = s_42_0;
        // D s_42_2: read-var take_in_el3:u8
        let s_42_2: bool = fn_state.take_in_el3;
        // N s_42_3: branch s_42_2 b61 b43
        if s_42_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_43_0: read-var route_to_el3:u8
        let s_43_0: bool = fn_state.route_to_el3;
        // D s_43_1: write-var gs#24322 <= s_43_0
        fn_state.gs_24322 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_44_0: read-var gs#24322:u8
        let s_44_0: bool = fn_state.gs_24322;
        // N s_44_1: branch s_44_0 b60 b45
        if s_44_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_45_0: read-var route_masked_to_el3:u8
        let s_45_0: bool = fn_state.route_masked_to_el3;
        // D s_45_1: write-var gs#24323 <= s_45_0
        fn_state.gs_24323 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_46_0: read-var gs#24323:u8
        let s_46_0: bool = fn_state.gs_24323;
        // N s_46_1: branch s_46_0 b59 b47
        if s_46_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_47_0: read-var take_in_el2_0:u8
        let s_47_0: bool = fn_state.take_in_el2_0;
        // N s_47_1: branch s_47_0 b58 b48
        if s_47_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_48_0: read-var route_to_el2:u8
        let s_48_0: bool = fn_state.route_to_el2;
        // D s_48_1: write-var gs#24325 <= s_48_0
        fn_state.gs_24325 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_49_0: read-var gs#24325:u8
        let s_49_0: bool = fn_state.gs_24325;
        // N s_49_1: branch s_49_0 b57 b50
        if s_49_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_50_0: read-var route_masked_to_el2:u8
        let s_50_0: bool = fn_state.route_masked_to_el2;
        // D s_50_1: write-var gs#24326 <= s_50_0
        fn_state.gs_24326 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_51_0: read-var gs#24326:u8
        let s_51_0: bool = fn_state.gs_24326;
        // N s_51_1: branch s_51_0 b56 b52
        if s_51_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_52_0: read-var take_in_el1_0:u8
        let s_52_0: bool = fn_state.take_in_el1_0;
        // N s_52_1: branch s_52_0 b55 b53
        if s_52_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var masked <= s_53_0
        fn_state.masked = s_53_0;
        // C s_53_2: const #2s : i64
        let s_53_2: i64 = 2;
        // C s_53_3: cast zx s_53_2 -> i
        let s_53_3: i128 = (i128::try_from(s_53_2).unwrap());
        // S s_53_4: call __UNKNOWN_bits(s_53_3)
        let s_53_4: Bits = u__UNKNOWN_bits(state, tracer, s_53_3);
        // S s_53_5: cast reint s_53_4 -> u8
        let s_53_5: u8 = (s_53_4.value() as u8);
        // D s_53_6: write-var target_el <= s_53_5
        fn_state.target_el = s_53_5;
        // N s_53_7: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_54_0: read-var masked:u8
        let s_54_0: bool = fn_state.masked;
        // D s_54_1: read-var target_el:u8
        let s_54_1: u8 = fn_state.target_el;
        // D s_54_2: create-product struct = ["s_54_0", "s_54_1"]
        let s_54_2: ProductTypea5cc8de4daab131c = ProductTypea5cc8de4daab131c {
            _0: s_54_0,
            _1: s_54_1,
        };
        // N s_54_3: return s_54_2
        return s_54_2;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var masked <= s_55_0
        fn_state.masked = s_55_0;
        // C s_55_2: const #440u : u32
        let s_55_2: u32 = 440;
        // D s_55_3: read-reg s_55_2:u8
        let s_55_3: u8 = {
            let value = state.read_register::<u8>(s_55_2 as isize);
            tracer.read_register(s_55_2 as isize, value);
            value
        };
        // D s_55_4: write-var target_el <= s_55_3
        fn_state.target_el = s_55_3;
        // N s_55_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var masked <= s_56_0
        fn_state.masked = s_56_0;
        // C s_56_2: const #432u : u32
        let s_56_2: u32 = 432;
        // D s_56_3: read-reg s_56_2:u8
        let s_56_3: u8 = {
            let value = state.read_register::<u8>(s_56_2 as isize);
            tracer.read_register(s_56_2 as isize, value);
            value
        };
        // D s_56_4: write-var target_el <= s_56_3
        fn_state.target_el = s_56_3;
        // N s_56_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#24326 <= s_57_0
        fn_state.gs_24326 = s_57_0;
        // N s_57_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#24325 <= s_58_0
        fn_state.gs_24325 = s_58_0;
        // N s_58_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var masked <= s_59_0
        fn_state.masked = s_59_0;
        // C s_59_2: const #424u : u32
        let s_59_2: u32 = 424;
        // D s_59_3: read-reg s_59_2:u8
        let s_59_3: u8 = {
            let value = state.read_register::<u8>(s_59_2 as isize);
            tracer.read_register(s_59_2 as isize, value);
            value
        };
        // D s_59_4: write-var target_el <= s_59_3
        fn_state.target_el = s_59_3;
        // N s_59_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#24323 <= s_60_0
        fn_state.gs_24323 = s_60_0;
        // N s_60_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#24322 <= s_61_0
        fn_state.gs_24322 = s_61_0;
        // N s_61_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_62_0: read-var masked:u8
        let s_62_0: bool = fn_state.masked;
        // D s_62_1: not s_62_0
        let s_62_1: bool = !s_62_0;
        // D s_62_2: write-var gs#24321 <= s_62_1
        fn_state.gs_24321 = s_62_1;
        // N s_62_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_63_0: read-var route_to_el3:u8
        let s_63_0: bool = fn_state.route_to_el3;
        // N s_63_1: branch s_63_0 b66 b64
        if s_63_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_64_0: read-var route_masked_to_el3:u8
        let s_64_0: bool = fn_state.route_masked_to_el3;
        // D s_64_1: write-var gs#24319 <= s_64_0
        fn_state.gs_24319 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_65_0: read-var gs#24319:u8
        let s_65_0: bool = fn_state.gs_24319;
        // D s_65_1: not s_65_0
        let s_65_1: bool = !s_65_0;
        // D s_65_2: write-var gs#24320 <= s_65_1
        fn_state.gs_24320 = s_65_1;
        // N s_65_3: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#24319 <= s_66_0
        fn_state.gs_24319 = s_66_0;
        // N s_66_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_67_0: read-var route_to_el2:u8
        let s_67_0: bool = fn_state.route_to_el2;
        // N s_67_1: branch s_67_0 b70 b68
        if s_67_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_68_0: read-var route_masked_to_el2:u8
        let s_68_0: bool = fn_state.route_masked_to_el2;
        // D s_68_1: write-var gs#24317 <= s_68_0
        fn_state.gs_24317 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_69_0: read-var gs#24317:u8
        let s_69_0: bool = fn_state.gs_24317;
        // D s_69_1: not s_69_0
        let s_69_1: bool = !s_69_0;
        // D s_69_2: write-var gs#24318 <= s_69_1
        fn_state.gs_24318 = s_69_1;
        // N s_69_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#24317 <= s_70_0
        fn_state.gs_24317 = s_70_0;
        // N s_70_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call IsInHost(s_71_0)
        let s_71_1: bool = IsInHost(state, tracer, s_71_0);
        // S s_71_2: not s_71_1
        let s_71_2: bool = !s_71_1;
        // D s_71_3: write-var gs#24315 <= s_71_2
        fn_state.gs_24315 = s_71_2;
        // N s_71_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#24316 <= s_72_0
        fn_state.gs_24316 = s_72_0;
        // N s_72_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_73_0: read-var masked:u8
        let s_73_0: bool = fn_state.masked;
        // D s_73_1: not s_73_0
        let s_73_1: bool = !s_73_0;
        // D s_73_2: write-var gs#24314 <= s_73_1
        fn_state.gs_24314 = s_73_1;
        // N s_73_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_74_0: read-var route_to_el3:u8
        let s_74_0: bool = fn_state.route_to_el3;
        // N s_74_1: branch s_74_0 b77 b75
        if s_74_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_75_0: read-var route_masked_to_el3:u8
        let s_75_0: bool = fn_state.route_masked_to_el3;
        // D s_75_1: write-var gs#24312 <= s_75_0
        fn_state.gs_24312 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_76_0: read-var gs#24312:u8
        let s_76_0: bool = fn_state.gs_24312;
        // D s_76_1: not s_76_0
        let s_76_1: bool = !s_76_0;
        // D s_76_2: write-var gs#24313 <= s_76_1
        fn_state.gs_24313 = s_76_1;
        // N s_76_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // D s_77_1: write-var gs#24312 <= s_77_0
        fn_state.gs_24312 = s_77_0;
        // N s_77_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_78_0: const #1u : u8
        let s_78_0: bool = true;
        // D s_78_1: write-var gs#24311 <= s_78_0
        fn_state.gs_24311 = s_78_0;
        // N s_78_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_79_0: read-var masked:u8
        let s_79_0: bool = fn_state.masked;
        // D s_79_1: not s_79_0
        let s_79_1: bool = !s_79_0;
        // D s_79_2: write-var gs#24310 <= s_79_1
        fn_state.gs_24310 = s_79_1;
        // N s_79_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call EL2Enabled(s_80_0)
        let s_80_1: bool = EL2Enabled(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b134 b81
        if s_80_1 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#24291 <= s_81_0
        fn_state.gs_24291 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_82_0: read-var gs#24291:u8
        let s_82_0: bool = fn_state.gs_24291;
        // N s_82_1: branch s_82_0 b133 b83
        if s_82_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#24292 <= s_83_0
        fn_state.gs_24292 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_84_0: read-var gs#24292:u8
        let s_84_0: bool = fn_state.gs_24292;
        // N s_84_1: branch s_84_0 b132 b85
        if s_84_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#24293 <= s_85_0
        fn_state.gs_24293 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_86_0: read-var gs#24293:u8
        let s_86_0: bool = fn_state.gs_24293;
        // N s_86_1: branch s_86_0 b116 b87
        if s_86_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#24299 <= s_87_0
        fn_state.gs_24299 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_88_0: read-var gs#24299:u8
        let s_88_0: bool = fn_state.gs_24299;
        // D s_88_1: write-var route_masked_to_el2 <= s_88_0
        fn_state.route_masked_to_el2 = s_88_0;
        // C s_88_2: const #424u : u32
        let s_88_2: u32 = 424;
        // D s_88_3: read-reg s_88_2:u8
        let s_88_3: u8 = {
            let value = state.read_register::<u8>(s_88_2 as isize);
            tracer.read_register(s_88_2 as isize, value);
            value
        };
        // C s_88_4: const #2u : u8
        let s_88_4: u8 = 2;
        // D s_88_5: cmp-lt s_88_3 s_88_4
        let s_88_5: bool = ((s_88_3) < (s_88_4));
        // N s_88_6: branch s_88_5 b115 b89
        if s_88_5 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#24301 <= s_89_0
        fn_state.gs_24301 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_90_0: read-var gs#24301:u8
        let s_90_0: bool = fn_state.gs_24301;
        // N s_90_1: branch s_90_0 b111 b91
        if s_90_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#24303 <= s_91_0
        fn_state.gs_24303 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_92_0: read-var gs#24303:u8
        let s_92_0: bool = fn_state.gs_24303;
        // N s_92_1: branch s_92_0 b95 b93
        if s_92_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#24309 <= s_93_0
        fn_state.gs_24309 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_94_0: read-var gs#24309:u8
        let s_94_0: bool = fn_state.gs_24309;
        // D s_94_1: write-var route_masked_to_el3 <= s_94_0
        fn_state.route_masked_to_el3 = s_94_0;
        // N s_94_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_95_0: const #16975u : u32
        let s_95_0: u32 = 16975;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: cast zx s_95_1 -> bv
        let s_95_2: Bits = Bits::new(s_95_1 as u128, 2u16);
        // C s_95_3: const #432u : u32
        let s_95_3: u32 = 432;
        // D s_95_4: read-reg s_95_3:u8
        let s_95_4: u8 = {
            let value = state.read_register::<u8>(s_95_3 as isize);
            tracer.read_register(s_95_3 as isize, value);
            value
        };
        // D s_95_5: cast zx s_95_4 -> bv
        let s_95_5: Bits = Bits::new(s_95_4 as u128, 2u16);
        // D s_95_6: cmp-eq s_95_2 s_95_5
        let s_95_6: bool = ((s_95_2) == (s_95_5));
        // N s_95_7: branch s_95_6 b110 b96
        if s_95_6 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_96_0: const #16975u : u32
        let s_96_0: u32 = 16975;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: u8 = {
            let value = state.read_register::<u8>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: cast zx s_96_1 -> bv
        let s_96_2: Bits = Bits::new(s_96_1 as u128, 2u16);
        // C s_96_3: const #440u : u32
        let s_96_3: u32 = 440;
        // D s_96_4: read-reg s_96_3:u8
        let s_96_4: u8 = {
            let value = state.read_register::<u8>(s_96_3 as isize);
            tracer.read_register(s_96_3 as isize, value);
            value
        };
        // D s_96_5: cast zx s_96_4 -> bv
        let s_96_5: Bits = Bits::new(s_96_4 as u128, 2u16);
        // D s_96_6: cmp-eq s_96_2 s_96_5
        let s_96_6: bool = ((s_96_2) == (s_96_5));
        // D s_96_7: write-var gs#24304 <= s_96_6
        fn_state.gs_24304 = s_96_6;
        // N s_96_8: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_97_0: read-var gs#24304:u8
        let s_97_0: bool = fn_state.gs_24304;
        // N s_97_1: branch s_97_0 b106 b98
        if s_97_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#24306 <= s_98_0
        fn_state.gs_24306 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_99_0: read-var gs#24306:u8
        let s_99_0: bool = fn_state.gs_24306;
        // N s_99_1: branch s_99_0 b105 b100
        if s_99_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_100_0: const #16975u : u32
        let s_100_0: u32 = 16975;
        // D s_100_1: read-reg s_100_0:u8
        let s_100_1: u8 = {
            let value = state.read_register::<u8>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: cast zx s_100_1 -> bv
        let s_100_2: Bits = Bits::new(s_100_1 as u128, 2u16);
        // C s_100_3: const #448u : u32
        let s_100_3: u32 = 448;
        // D s_100_4: read-reg s_100_3:u8
        let s_100_4: u8 = {
            let value = state.read_register::<u8>(s_100_3 as isize);
            tracer.read_register(s_100_3 as isize, value);
            value
        };
        // D s_100_5: cast zx s_100_4 -> bv
        let s_100_5: Bits = Bits::new(s_100_4 as u128, 2u16);
        // D s_100_6: cmp-eq s_100_2 s_100_5
        let s_100_6: bool = ((s_100_2) == (s_100_5));
        // N s_100_7: branch s_100_6 b104 b101
        if s_100_6 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#24307 <= s_101_0
        fn_state.gs_24307 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_102_0: read-var gs#24307:u8
        let s_102_0: bool = fn_state.gs_24307;
        // D s_102_1: write-var gs#24308 <= s_102_0
        fn_state.gs_24308 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_103_0: read-var gs#24308:u8
        let s_103_0: bool = fn_state.gs_24308;
        // D s_103_1: write-var gs#24309 <= s_103_0
        fn_state.gs_24309 = s_103_0;
        // N s_103_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_104_0: read-var masked:u8
        let s_104_0: bool = fn_state.masked;
        // D s_104_1: write-var gs#24307 <= s_104_0
        fn_state.gs_24307 = s_104_0;
        // N s_104_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_105_0: const #1u : u8
        let s_105_0: bool = true;
        // D s_105_1: write-var gs#24308 <= s_105_0
        fn_state.gs_24308 = s_105_0;
        // N s_105_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_106_0: const #16968u : u32
        let s_106_0: u32 = 16968;
        // D s_106_1: read-reg s_106_0:u8
        let s_106_1: bool = {
            let value = state.read_register::<bool>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: cast zx s_106_1 -> bv
        let s_106_2: Bits = Bits::new(s_106_1 as u128, 1u16);
        // C s_106_3: const #1u : u8
        let s_106_3: bool = true;
        // C s_106_4: cast zx s_106_3 -> bv
        let s_106_4: Bits = Bits::new(s_106_3 as u128, 1u16);
        // D s_106_5: cmp-eq s_106_2 s_106_4
        let s_106_5: bool = ((s_106_2) == (s_106_4));
        // N s_106_6: branch s_106_5 b109 b107
        if s_106_5 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_107_0: read-var masked:u8
        let s_107_0: bool = fn_state.masked;
        // D s_107_1: write-var gs#24305 <= s_107_0
        fn_state.gs_24305 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_108_0: read-var gs#24305:u8
        let s_108_0: bool = fn_state.gs_24305;
        // D s_108_1: write-var gs#24306 <= s_108_0
        fn_state.gs_24306 = s_108_0;
        // N s_108_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_109_0: const #1u : u8
        let s_109_0: bool = true;
        // D s_109_1: write-var gs#24305 <= s_109_0
        fn_state.gs_24305 = s_109_0;
        // N s_109_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_110_0: const #1u : u8
        let s_110_0: bool = true;
        // D s_110_1: write-var gs#24304 <= s_110_0
        fn_state.gs_24304 = s_110_0;
        // N s_110_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_111_0: read-var route_to_el2:u8
        let s_111_0: bool = fn_state.route_to_el2;
        // N s_111_1: branch s_111_0 b114 b112
        if s_111_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_112_0: read-var route_masked_to_el2:u8
        let s_112_0: bool = fn_state.route_masked_to_el2;
        // D s_112_1: write-var gs#24302 <= s_112_0
        fn_state.gs_24302 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_113_0: read-var gs#24302:u8
        let s_113_0: bool = fn_state.gs_24302;
        // D s_113_1: not s_113_0
        let s_113_1: bool = !s_113_0;
        // D s_113_2: write-var gs#24303 <= s_113_1
        fn_state.gs_24303 = s_113_1;
        // N s_113_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#24302 <= s_114_0
        fn_state.gs_24302 = s_114_0;
        // N s_114_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_115_0: const #90704u : u32
        let s_115_0: u32 = 90704;
        // D s_115_1: read-reg s_115_0:struct
        let s_115_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // D s_115_2: call _get_SCR_EL3_Type_TMEA(s_115_1)
        let s_115_2: bool = u_get_SCR_EL3_Type_TMEA(state, tracer, s_115_1);
        // D s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // C s_115_4: const #1u : u8
        let s_115_4: bool = true;
        // C s_115_5: cast zx s_115_4 -> bv
        let s_115_5: Bits = Bits::new(s_115_4 as u128, 1u16);
        // D s_115_6: cmp-eq s_115_3 s_115_5
        let s_115_6: bool = ((s_115_3) == (s_115_5));
        // D s_115_7: write-var gs#24301 <= s_115_6
        fn_state.gs_24301 = s_115_6;
        // N s_115_8: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_116_0: const #16975u : u32
        let s_116_0: u32 = 16975;
        // D s_116_1: read-reg s_116_0:u8
        let s_116_1: u8 = {
            let value = state.read_register::<u8>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // D s_116_2: cast zx s_116_1 -> bv
        let s_116_2: Bits = Bits::new(s_116_1 as u128, 2u16);
        // C s_116_3: const #440u : u32
        let s_116_3: u32 = 440;
        // D s_116_4: read-reg s_116_3:u8
        let s_116_4: u8 = {
            let value = state.read_register::<u8>(s_116_3 as isize);
            tracer.read_register(s_116_3 as isize, value);
            value
        };
        // D s_116_5: cast zx s_116_4 -> bv
        let s_116_5: Bits = Bits::new(s_116_4 as u128, 2u16);
        // D s_116_6: cmp-eq s_116_2 s_116_5
        let s_116_6: bool = ((s_116_2) == (s_116_5));
        // N s_116_7: branch s_116_6 b128 b117
        if s_116_6 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#24295 <= s_117_0
        fn_state.gs_24295 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_118_0: read-var gs#24295:u8
        let s_118_0: bool = fn_state.gs_24295;
        // N s_118_1: branch s_118_0 b127 b119
        if s_118_0 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_119_0: const #16975u : u32
        let s_119_0: u32 = 16975;
        // D s_119_1: read-reg s_119_0:u8
        let s_119_1: u8 = {
            let value = state.read_register::<u8>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // D s_119_2: cast zx s_119_1 -> bv
        let s_119_2: Bits = Bits::new(s_119_1 as u128, 2u16);
        // C s_119_3: const #448u : u32
        let s_119_3: u32 = 448;
        // D s_119_4: read-reg s_119_3:u8
        let s_119_4: u8 = {
            let value = state.read_register::<u8>(s_119_3 as isize);
            tracer.read_register(s_119_3 as isize, value);
            value
        };
        // D s_119_5: cast zx s_119_4 -> bv
        let s_119_5: Bits = Bits::new(s_119_4 as u128, 2u16);
        // D s_119_6: cmp-eq s_119_2 s_119_5
        let s_119_6: bool = ((s_119_2) == (s_119_5));
        // N s_119_7: branch s_119_6 b126 b120
        if s_119_6 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#24296 <= s_120_0
        fn_state.gs_24296 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_121_0: read-var gs#24296:u8
        let s_121_0: bool = fn_state.gs_24296;
        // N s_121_1: branch s_121_0 b125 b122
        if s_121_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#24297 <= s_122_0
        fn_state.gs_24297 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_123_0: read-var gs#24297:u8
        let s_123_0: bool = fn_state.gs_24297;
        // D s_123_1: write-var gs#24298 <= s_123_0
        fn_state.gs_24298 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_124_0: read-var gs#24298:u8
        let s_124_0: bool = fn_state.gs_24298;
        // D s_124_1: write-var gs#24299 <= s_124_0
        fn_state.gs_24299 = s_124_0;
        // N s_124_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_125_0: const #() : ()
        let s_125_0: () = ();
        // S s_125_1: call IsInHost(s_125_0)
        let s_125_1: bool = IsInHost(state, tracer, s_125_0);
        // S s_125_2: not s_125_1
        let s_125_2: bool = !s_125_1;
        // D s_125_3: write-var gs#24297 <= s_125_2
        fn_state.gs_24297 = s_125_2;
        // N s_125_4: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_126_0: read-var masked:u8
        let s_126_0: bool = fn_state.masked;
        // D s_126_1: write-var gs#24296 <= s_126_0
        fn_state.gs_24296 = s_126_0;
        // N s_126_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // D s_127_1: write-var gs#24298 <= s_127_0
        fn_state.gs_24298 = s_127_0;
        // N s_127_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_128_0: const #16968u : u32
        let s_128_0: u32 = 16968;
        // D s_128_1: read-reg s_128_0:u8
        let s_128_1: bool = {
            let value = state.read_register::<bool>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // D s_128_2: cast zx s_128_1 -> bv
        let s_128_2: Bits = Bits::new(s_128_1 as u128, 1u16);
        // C s_128_3: const #1u : u8
        let s_128_3: bool = true;
        // C s_128_4: cast zx s_128_3 -> bv
        let s_128_4: Bits = Bits::new(s_128_3 as u128, 1u16);
        // D s_128_5: cmp-eq s_128_2 s_128_4
        let s_128_5: bool = ((s_128_2) == (s_128_4));
        // N s_128_6: branch s_128_5 b131 b129
        if s_128_5 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_129_0: read-var masked:u8
        let s_129_0: bool = fn_state.masked;
        // D s_129_1: write-var gs#24294 <= s_129_0
        fn_state.gs_24294 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_130_0: read-var gs#24294:u8
        let s_130_0: bool = fn_state.gs_24294;
        // D s_130_1: write-var gs#24295 <= s_130_0
        fn_state.gs_24295 = s_130_0;
        // N s_130_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_131_0: const #1u : u8
        let s_131_0: bool = true;
        // D s_131_1: write-var gs#24294 <= s_131_0
        fn_state.gs_24294 = s_131_0;
        // N s_131_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_132_0: const #22528u : u32
        let s_132_0: u32 = 22528;
        // D s_132_1: read-reg s_132_0:struct
        let s_132_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // D s_132_2: call _get_HCRX_EL2_Type_TMEA(s_132_1)
        let s_132_2: bool = u_get_HCRX_EL2_Type_TMEA(state, tracer, s_132_1);
        // D s_132_3: cast zx s_132_2 -> bv
        let s_132_3: Bits = Bits::new(s_132_2 as u128, 1u16);
        // C s_132_4: const #1u : u8
        let s_132_4: bool = true;
        // C s_132_5: cast zx s_132_4 -> bv
        let s_132_5: Bits = Bits::new(s_132_4 as u128, 1u16);
        // D s_132_6: cmp-eq s_132_3 s_132_5
        let s_132_6: bool = ((s_132_3) == (s_132_5));
        // D s_132_7: write-var gs#24293 <= s_132_6
        fn_state.gs_24293 = s_132_6;
        // N s_132_8: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call IsHCRXEL2Enabled(s_133_0)
        let s_133_1: bool = IsHCRXEL2Enabled(state, tracer, s_133_0);
        // D s_133_2: write-var gs#24292 <= s_133_1
        fn_state.gs_24292 = s_133_1;
        // N s_133_3: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_134_0: read-var route_to_el3:u8
        let s_134_0: bool = fn_state.route_to_el3;
        // D s_134_1: not s_134_0
        let s_134_1: bool = !s_134_0;
        // D s_134_2: write-var gs#24291 <= s_134_1
        fn_state.gs_24291 = s_134_1;
        // N s_134_3: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_135_0: const #90704u : u32
        let s_135_0: u32 = 90704;
        // D s_135_1: read-reg s_135_0:struct
        let s_135_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_135_0 as isize);
            tracer.read_register(s_135_0 as isize, value);
            value
        };
        // D s_135_2: call _get_SCR_EL3_Type_NMEA(s_135_1)
        let s_135_2: bool = u_get_SCR_EL3_Type_NMEA(state, tracer, s_135_1);
        // C s_135_3: const #() : ()
        let s_135_3: () = ();
        // S s_135_4: call EffectiveEA(s_135_3)
        let s_135_4: bool = EffectiveEA(state, tracer, s_135_3);
        // D s_135_5: cast zx s_135_2 -> bv
        let s_135_5: Bits = Bits::new(s_135_2 as u128, 1u16);
        // S s_135_6: cast zx s_135_4 -> bv
        let s_135_6: Bits = Bits::new(s_135_4 as u128, 1u16);
        // D s_135_7: and s_135_5 s_135_6
        let s_135_7: Bits = ((s_135_5) & (s_135_6));
        // D s_135_8: cast reint s_135_7 -> u8
        let s_135_8: bool = ((s_135_7.value()) != 0);
        // D s_135_9: write-var u#1261 <= s_135_8
        fn_state.u_1261 = s_135_8;
        // D s_135_10: read-var masked:u8
        let s_135_10: bool = fn_state.masked;
        // N s_135_11: branch s_135_10 b138 b136
        if s_135_10 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#24279 <= s_136_0
        fn_state.gs_24279 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_137_0: read-var gs#24279:u8
        let s_137_0: bool = fn_state.gs_24279;
        // D s_137_1: write-var masked <= s_137_0
        fn_state.masked = s_137_0;
        // N s_137_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_138_0: read-var u#1261:u8
        let s_138_0: bool = fn_state.u_1261;
        // D s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 1u16);
        // C s_138_2: const #0u : u8
        let s_138_2: bool = false;
        // C s_138_3: cast zx s_138_2 -> bv
        let s_138_3: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_4: cmp-eq s_138_1 s_138_3
        let s_138_4: bool = ((s_138_1) == (s_138_3));
        // D s_138_5: write-var gs#24279 <= s_138_4
        fn_state.gs_24279 = s_138_4;
        // N s_138_6: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_139_0: const #16975u : u32
        let s_139_0: u32 = 16975;
        // D s_139_1: read-reg s_139_0:u8
        let s_139_1: u8 = {
            let value = state.read_register::<u8>(s_139_0 as isize);
            tracer.read_register(s_139_0 as isize, value);
            value
        };
        // D s_139_2: cast zx s_139_1 -> bv
        let s_139_2: Bits = Bits::new(s_139_1 as u128, 2u16);
        // C s_139_3: const #424u : u32
        let s_139_3: u32 = 424;
        // D s_139_4: read-reg s_139_3:u8
        let s_139_4: u8 = {
            let value = state.read_register::<u8>(s_139_3 as isize);
            tracer.read_register(s_139_3 as isize, value);
            value
        };
        // D s_139_5: cast zx s_139_4 -> bv
        let s_139_5: Bits = Bits::new(s_139_4 as u128, 2u16);
        // D s_139_6: cmp-eq s_139_2 s_139_5
        let s_139_6: bool = ((s_139_2) == (s_139_5));
        // D s_139_7: write-var gs#24278 <= s_139_6
        fn_state.gs_24278 = s_139_6;
        // N s_139_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_140_0: const #16975u : u32
        let s_140_0: u32 = 16975;
        // D s_140_1: read-reg s_140_0:u8
        let s_140_1: u8 = {
            let value = state.read_register::<u8>(s_140_0 as isize);
            tracer.read_register(s_140_0 as isize, value);
            value
        };
        // D s_140_2: write-var ga#18911 <= s_140_1
        fn_state.ga_18911 = s_140_1;
        // D s_140_3: read-var ga#18911:u8
        let s_140_3: u8 = fn_state.ga_18911;
        // D s_140_4: cast zx s_140_3 -> bv
        let s_140_4: Bits = Bits::new(s_140_3 as u128, 2u16);
        // C s_140_5: const #424u : u32
        let s_140_5: u32 = 424;
        // D s_140_6: read-reg s_140_5:u8
        let s_140_6: u8 = {
            let value = state.read_register::<u8>(s_140_5 as isize);
            tracer.read_register(s_140_5 as isize, value);
            value
        };
        // D s_140_7: cast zx s_140_6 -> bv
        let s_140_7: Bits = Bits::new(s_140_6 as u128, 2u16);
        // D s_140_8: cmp-eq s_140_4 s_140_7
        let s_140_8: bool = ((s_140_4) == (s_140_7));
        // D s_140_9: not s_140_8
        let s_140_9: bool = !s_140_8;
        // N s_140_10: branch s_140_9 b146 b141
        if s_140_9 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_141_0: const #90704u : u32
        let s_141_0: u32 = 90704;
        // D s_141_1: read-reg s_141_0:struct
        let s_141_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_141_0 as isize);
            tracer.read_register(s_141_0 as isize, value);
            value
        };
        // D s_141_2: call _get_SCR_EL3_Type_NMEA(s_141_1)
        let s_141_2: bool = u_get_SCR_EL3_Type_NMEA(state, tracer, s_141_1);
        // D s_141_3: write-var nmea_bit <= s_141_2
        fn_state.nmea_bit = s_141_2;
        // N s_141_4: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_142_0: read-var masked:u8
        let s_142_0: bool = fn_state.masked;
        // N s_142_1: branch s_142_0 b145 b143
        if s_142_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#24287 <= s_143_0
        fn_state.gs_24287 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_144_0: read-var gs#24287:u8
        let s_144_0: bool = fn_state.gs_24287;
        // D s_144_1: write-var masked <= s_144_0
        fn_state.masked = s_144_0;
        // N s_144_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_145_0: read-var nmea_bit:u8
        let s_145_0: bool = fn_state.nmea_bit;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #0u : u8
        let s_145_2: bool = false;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // D s_145_5: write-var gs#24287 <= s_145_4
        fn_state.gs_24287 = s_145_4;
        // N s_145_6: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_146_0: read-var ga#18911:u8
        let s_146_0: u8 = fn_state.ga_18911;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 2u16);
        // C s_146_2: const #432u : u32
        let s_146_2: u32 = 432;
        // D s_146_3: read-reg s_146_2:u8
        let s_146_3: u8 = {
            let value = state.read_register::<u8>(s_146_2 as isize);
            tracer.read_register(s_146_2 as isize, value);
            value
        };
        // D s_146_4: cast zx s_146_3 -> bv
        let s_146_4: Bits = Bits::new(s_146_3 as u128, 2u16);
        // D s_146_5: cmp-eq s_146_1 s_146_4
        let s_146_5: bool = ((s_146_1) == (s_146_4));
        // D s_146_6: not s_146_5
        let s_146_6: bool = !s_146_5;
        // N s_146_7: branch s_146_6 b151 b147
        if s_146_6 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_147_0: const #() : ()
        let s_147_0: () = ();
        // S s_147_1: call IsSCTLR2EL2Enabled(s_147_0)
        let s_147_1: bool = IsSCTLR2EL2Enabled(state, tracer, s_147_0);
        // N s_147_2: branch s_147_1 b150 b148
        if s_147_1 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var nmea_bit <= s_148_0
        fn_state.nmea_bit = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_149_0: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_150_0: const #102680u : u32
        let s_150_0: u32 = 102680;
        // D s_150_1: read-reg s_150_0:struct
        let s_150_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // D s_150_2: call _get_SCTLR2_EL2_Type_NMEA(s_150_1)
        let s_150_2: bool = u_get_SCTLR2_EL2_Type_NMEA(state, tracer, s_150_1);
        // D s_150_3: write-var nmea_bit <= s_150_2
        fn_state.nmea_bit = s_150_2;
        // N s_150_4: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_151_0: read-var ga#18911:u8
        let s_151_0: u8 = fn_state.ga_18911;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 2u16);
        // C s_151_2: const #440u : u32
        let s_151_2: u32 = 440;
        // D s_151_3: read-reg s_151_2:u8
        let s_151_3: u8 = {
            let value = state.read_register::<u8>(s_151_2 as isize);
            tracer.read_register(s_151_2 as isize, value);
            value
        };
        // D s_151_4: cast zx s_151_3 -> bv
        let s_151_4: Bits = Bits::new(s_151_3 as u128, 2u16);
        // D s_151_5: cmp-eq s_151_1 s_151_4
        let s_151_5: bool = ((s_151_1) == (s_151_4));
        // D s_151_6: not s_151_5
        let s_151_6: bool = !s_151_5;
        // N s_151_7: branch s_151_6 b156 b152
        if s_151_6 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_152_0: const #() : ()
        let s_152_0: () = ();
        // S s_152_1: call IsSCTLR2EL1Enabled(s_152_0)
        let s_152_1: bool = IsSCTLR2EL1Enabled(state, tracer, s_152_0);
        // N s_152_2: branch s_152_1 b155 b153
        if s_152_1 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_153_0: const #0u : u8
        let s_153_0: bool = false;
        // D s_153_1: write-var nmea_bit <= s_153_0
        fn_state.nmea_bit = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_154_0: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_155_0: const #11720u : u32
        let s_155_0: u32 = 11720;
        // D s_155_1: read-reg s_155_0:struct
        let s_155_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_155_0 as isize);
            tracer.read_register(s_155_0 as isize, value);
            value
        };
        // D s_155_2: call _get_SCTLR2_EL1_Type_NMEA(s_155_1)
        let s_155_2: bool = u_get_SCTLR2_EL1_Type_NMEA(state, tracer, s_155_1);
        // D s_155_3: write-var nmea_bit <= s_155_2
        fn_state.nmea_bit = s_155_2;
        // N s_155_4: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_156_0: read-var ga#18911:u8
        let s_156_0: u8 = fn_state.ga_18911;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 2u16);
        // C s_156_2: const #448u : u32
        let s_156_2: u32 = 448;
        // D s_156_3: read-reg s_156_2:u8
        let s_156_3: u8 = {
            let value = state.read_register::<u8>(s_156_2 as isize);
            tracer.read_register(s_156_2 as isize, value);
            value
        };
        // D s_156_4: cast zx s_156_3 -> bv
        let s_156_4: Bits = Bits::new(s_156_3 as u128, 2u16);
        // D s_156_5: cmp-eq s_156_1 s_156_4
        let s_156_5: bool = ((s_156_1) == (s_156_4));
        // D s_156_6: not s_156_5
        let s_156_6: bool = !s_156_5;
        // N s_156_7: branch s_156_6 b166 b157
        if s_156_6 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_157_0: const #() : ()
        let s_157_0: () = ();
        // S s_157_1: call IsInHost(s_157_0)
        let s_157_1: bool = IsInHost(state, tracer, s_157_0);
        // N s_157_2: branch s_157_1 b162 b158
        if s_157_1 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_158_0: const #() : ()
        let s_158_0: () = ();
        // S s_158_1: call IsSCTLR2EL1Enabled(s_158_0)
        let s_158_1: bool = IsSCTLR2EL1Enabled(state, tracer, s_158_0);
        // N s_158_2: branch s_158_1 b161 b159
        if s_158_1 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_159_0: const #0u : u8
        let s_159_0: bool = false;
        // D s_159_1: write-var nmea_bit <= s_159_0
        fn_state.nmea_bit = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_160_0: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_161_0: const #11720u : u32
        let s_161_0: u32 = 11720;
        // D s_161_1: read-reg s_161_0:struct
        let s_161_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_161_0 as isize);
            tracer.read_register(s_161_0 as isize, value);
            value
        };
        // D s_161_2: call _get_SCTLR2_EL1_Type_NMEA(s_161_1)
        let s_161_2: bool = u_get_SCTLR2_EL1_Type_NMEA(state, tracer, s_161_1);
        // D s_161_3: write-var nmea_bit <= s_161_2
        fn_state.nmea_bit = s_161_2;
        // N s_161_4: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_162_0: const #() : ()
        let s_162_0: () = ();
        // S s_162_1: call IsSCTLR2EL2Enabled(s_162_0)
        let s_162_1: bool = IsSCTLR2EL2Enabled(state, tracer, s_162_0);
        // N s_162_2: branch s_162_1 b165 b163
        if s_162_1 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_163_0: const #0u : u8
        let s_163_0: bool = false;
        // D s_163_1: write-var nmea_bit <= s_163_0
        fn_state.nmea_bit = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_164_0: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_165_0: const #102680u : u32
        let s_165_0: u32 = 102680;
        // D s_165_1: read-reg s_165_0:struct
        let s_165_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_165_0 as isize);
            tracer.read_register(s_165_0 as isize, value);
            value
        };
        // D s_165_2: call _get_SCTLR2_EL2_Type_NMEA(s_165_1)
        let s_165_2: bool = u_get_SCTLR2_EL2_Type_NMEA(state, tracer, s_165_1);
        // D s_165_3: write-var nmea_bit <= s_165_2
        fn_state.nmea_bit = s_165_2;
        // N s_165_4: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_166_0: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_167_0: const #1u : u8
        let s_167_0: bool = true;
        // D s_167_1: write-var gs#24265 <= s_167_0
        fn_state.gs_24265 = s_167_0;
        // N s_167_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_168_0: read-var ga#18894:u8
        let s_168_0: u8 = fn_state.ga_18894;
        // D s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 2u16);
        // C s_168_2: const #432u : u32
        let s_168_2: u32 = 432;
        // D s_168_3: read-reg s_168_2:u8
        let s_168_3: u8 = {
            let value = state.read_register::<u8>(s_168_2 as isize);
            tracer.read_register(s_168_2 as isize, value);
            value
        };
        // D s_168_4: cast zx s_168_3 -> bv
        let s_168_4: Bits = Bits::new(s_168_3 as u128, 2u16);
        // D s_168_5: cmp-eq s_168_1 s_168_4
        let s_168_5: bool = ((s_168_1) == (s_168_4));
        // D s_168_6: not s_168_5
        let s_168_6: bool = !s_168_5;
        // N s_168_7: branch s_168_6 b176 b169
        if s_168_6 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_169_0: read-var route_to_el3:u8
        let s_169_0: bool = fn_state.route_to_el3;
        // D s_169_1: not s_169_0
        let s_169_1: bool = !s_169_0;
        // N s_169_2: branch s_169_1 b172 b170
        if s_169_1 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_170_0: const #0u : u8
        let s_170_0: bool = false;
        // D s_170_1: write-var gs#24268 <= s_170_0
        fn_state.gs_24268 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_171_0: read-var gs#24268:u8
        let s_171_0: bool = fn_state.gs_24268;
        // D s_171_1: write-var masked <= s_171_0
        fn_state.masked = s_171_0;
        // N s_171_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_172_0: const #102552u : u32
        let s_172_0: u32 = 102552;
        // D s_172_1: read-reg s_172_0:struct
        let s_172_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // D s_172_2: call _get_HCR_EL2_Type_TGE(s_172_1)
        let s_172_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_172_1);
        // C s_172_3: const #102552u : u32
        let s_172_3: u32 = 102552;
        // D s_172_4: read-reg s_172_3:struct
        let s_172_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_172_3 as isize);
            tracer.read_register(s_172_3 as isize, value);
            value
        };
        // D s_172_5: call _get_HCR_EL2_Type_AMO(s_172_4)
        let s_172_5: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_172_4);
        // D s_172_6: cast zx s_172_2 -> bv
        let s_172_6: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_7: cast zx s_172_5 -> bv
        let s_172_7: Bits = Bits::new(s_172_5 as u128, 1u16);
        // D s_172_8: cast reint s_172_6 -> u128
        let s_172_8: u128 = (s_172_6.value() as u128);
        // D s_172_9: size-of s_172_6
        let s_172_9: u16 = s_172_6.length();
        // D s_172_10: cast reint s_172_7 -> u128
        let s_172_10: u128 = (s_172_7.value() as u128);
        // D s_172_11: size-of s_172_7
        let s_172_11: u16 = s_172_7.length();
        // D s_172_12: lsl s_172_8 s_172_11
        let s_172_12: u128 = s_172_8 << s_172_11;
        // D s_172_13: or s_172_12 s_172_10
        let s_172_13: u128 = ((s_172_12) | (s_172_10));
        // D s_172_14: add s_172_9 s_172_11
        let s_172_14: u16 = (s_172_9 + s_172_11);
        // D s_172_15: create-bits s_172_13 s_172_14
        let s_172_15: Bits = Bits::new(s_172_13, s_172_14);
        // D s_172_16: cast reint s_172_15 -> u8
        let s_172_16: u8 = (s_172_15.value() as u8);
        // D s_172_17: cast zx s_172_16 -> bv
        let s_172_17: Bits = Bits::new(s_172_16 as u128, 2u16);
        // C s_172_18: const #0u : u8
        let s_172_18: u8 = 0;
        // C s_172_19: cast zx s_172_18 -> bv
        let s_172_19: Bits = Bits::new(s_172_18 as u128, 2u16);
        // D s_172_20: cmp-eq s_172_17 s_172_19
        let s_172_20: bool = ((s_172_17) == (s_172_19));
        // N s_172_21: branch s_172_20 b175 b173
        if s_172_20 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_173_0: const #16968u : u32
        let s_173_0: u32 = 16968;
        // D s_173_1: read-reg s_173_0:u8
        let s_173_1: bool = {
            let value = state.read_register::<bool>(s_173_0 as isize);
            tracer.read_register(s_173_0 as isize, value);
            value
        };
        // D s_173_2: cast zx s_173_1 -> bv
        let s_173_2: Bits = Bits::new(s_173_1 as u128, 1u16);
        // C s_173_3: const #1u : u8
        let s_173_3: bool = true;
        // C s_173_4: cast zx s_173_3 -> bv
        let s_173_4: Bits = Bits::new(s_173_3 as u128, 1u16);
        // D s_173_5: cmp-eq s_173_2 s_173_4
        let s_173_5: bool = ((s_173_2) == (s_173_4));
        // D s_173_6: write-var gs#24267 <= s_173_5
        fn_state.gs_24267 = s_173_5;
        // N s_173_7: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_174_0: read-var gs#24267:u8
        let s_174_0: bool = fn_state.gs_24267;
        // D s_174_1: write-var gs#24268 <= s_174_0
        fn_state.gs_24268 = s_174_0;
        // N s_174_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_175_0: const #1u : u8
        let s_175_0: bool = true;
        // D s_175_1: write-var gs#24267 <= s_175_0
        fn_state.gs_24267 = s_175_0;
        // N s_175_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_176_0: read-var ga#18894:u8
        let s_176_0: u8 = fn_state.ga_18894;
        // D s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 2u16);
        // C s_176_2: const #440u : u32
        let s_176_2: u32 = 440;
        // D s_176_3: read-reg s_176_2:u8
        let s_176_3: u8 = {
            let value = state.read_register::<u8>(s_176_2 as isize);
            tracer.read_register(s_176_2 as isize, value);
            value
        };
        // D s_176_4: cast zx s_176_3 -> bv
        let s_176_4: Bits = Bits::new(s_176_3 as u128, 2u16);
        // D s_176_5: cmp-eq s_176_1 s_176_4
        let s_176_5: bool = ((s_176_1) == (s_176_4));
        // D s_176_6: not s_176_5
        let s_176_6: bool = !s_176_5;
        // N s_176_7: branch s_176_6 b184 b177
        if s_176_6 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_177_0: read-var route_to_el3:u8
        let s_177_0: bool = fn_state.route_to_el3;
        // D s_177_1: not s_177_0
        let s_177_1: bool = !s_177_0;
        // N s_177_2: branch s_177_1 b183 b178
        if s_177_1 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_178_0: const #0u : u8
        let s_178_0: bool = false;
        // D s_178_1: write-var gs#24270 <= s_178_0
        fn_state.gs_24270 = s_178_0;
        // N s_178_2: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_179_0: read-var gs#24270:u8
        let s_179_0: bool = fn_state.gs_24270;
        // N s_179_1: branch s_179_0 b182 b180
        if s_179_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_180(state, tracer, fn_state);
        };
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_180_0: const #0u : u8
        let s_180_0: bool = false;
        // D s_180_1: write-var gs#24271 <= s_180_0
        fn_state.gs_24271 = s_180_0;
        // N s_180_2: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_181_0: read-var gs#24271:u8
        let s_181_0: bool = fn_state.gs_24271;
        // D s_181_1: write-var masked <= s_181_0
        fn_state.masked = s_181_0;
        // N s_181_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_182_0: const #16968u : u32
        let s_182_0: u32 = 16968;
        // D s_182_1: read-reg s_182_0:u8
        let s_182_1: bool = {
            let value = state.read_register::<bool>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // D s_182_2: cast zx s_182_1 -> bv
        let s_182_2: Bits = Bits::new(s_182_1 as u128, 1u16);
        // C s_182_3: const #1u : u8
        let s_182_3: bool = true;
        // C s_182_4: cast zx s_182_3 -> bv
        let s_182_4: Bits = Bits::new(s_182_3 as u128, 1u16);
        // D s_182_5: cmp-eq s_182_2 s_182_4
        let s_182_5: bool = ((s_182_2) == (s_182_4));
        // D s_182_6: write-var gs#24271 <= s_182_5
        fn_state.gs_24271 = s_182_5;
        // N s_182_7: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_183_0: read-var route_to_el2:u8
        let s_183_0: bool = fn_state.route_to_el2;
        // D s_183_1: not s_183_0
        let s_183_1: bool = !s_183_0;
        // D s_183_2: write-var gs#24270 <= s_183_1
        fn_state.gs_24270 = s_183_1;
        // N s_183_3: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_184_0: read-var ga#18894:u8
        let s_184_0: u8 = fn_state.ga_18894;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 2u16);
        // C s_184_2: const #448u : u32
        let s_184_2: u32 = 448;
        // D s_184_3: read-reg s_184_2:u8
        let s_184_3: u8 = {
            let value = state.read_register::<u8>(s_184_2 as isize);
            tracer.read_register(s_184_2 as isize, value);
            value
        };
        // D s_184_4: cast zx s_184_3 -> bv
        let s_184_4: Bits = Bits::new(s_184_3 as u128, 2u16);
        // D s_184_5: cmp-eq s_184_1 s_184_4
        let s_184_5: bool = ((s_184_1) == (s_184_4));
        // D s_184_6: not s_184_5
        let s_184_6: bool = !s_184_5;
        // N s_184_7: branch s_184_6 b192 b185
        if s_184_6 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_185_0: read-var route_to_el3:u8
        let s_185_0: bool = fn_state.route_to_el3;
        // D s_185_1: not s_185_0
        let s_185_1: bool = !s_185_0;
        // N s_185_2: branch s_185_1 b191 b186
        if s_185_1 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_186_0: const #0u : u8
        let s_186_0: bool = false;
        // D s_186_1: write-var gs#24273 <= s_186_0
        fn_state.gs_24273 = s_186_0;
        // N s_186_2: jump b187
        return block_187(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_187_0: read-var gs#24273:u8
        let s_187_0: bool = fn_state.gs_24273;
        // N s_187_1: branch s_187_0 b190 b188
        if s_187_0 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_188(state, tracer, fn_state);
        };
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_188_0: const #0u : u8
        let s_188_0: bool = false;
        // D s_188_1: write-var gs#24274 <= s_188_0
        fn_state.gs_24274 = s_188_0;
        // N s_188_2: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_189_0: read-var gs#24274:u8
        let s_189_0: bool = fn_state.gs_24274;
        // D s_189_1: write-var masked <= s_189_0
        fn_state.masked = s_189_0;
        // N s_189_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_190_0: const #16968u : u32
        let s_190_0: u32 = 16968;
        // D s_190_1: read-reg s_190_0:u8
        let s_190_1: bool = {
            let value = state.read_register::<bool>(s_190_0 as isize);
            tracer.read_register(s_190_0 as isize, value);
            value
        };
        // D s_190_2: cast zx s_190_1 -> bv
        let s_190_2: Bits = Bits::new(s_190_1 as u128, 1u16);
        // C s_190_3: const #1u : u8
        let s_190_3: bool = true;
        // C s_190_4: cast zx s_190_3 -> bv
        let s_190_4: Bits = Bits::new(s_190_3 as u128, 1u16);
        // D s_190_5: cmp-eq s_190_2 s_190_4
        let s_190_5: bool = ((s_190_2) == (s_190_4));
        // D s_190_6: write-var gs#24274 <= s_190_5
        fn_state.gs_24274 = s_190_5;
        // N s_190_7: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_191_0: read-var route_to_el2:u8
        let s_191_0: bool = fn_state.route_to_el2;
        // D s_191_1: not s_191_0
        let s_191_1: bool = !s_191_0;
        // D s_191_2: write-var gs#24273 <= s_191_1
        fn_state.gs_24273 = s_191_1;
        // N s_191_3: jump b187
        return block_187(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // N s_192_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_193_0: const #() : ()
        let s_193_0: () = ();
        // S s_193_1: call IsInHost(s_193_0)
        let s_193_1: bool = IsInHost(state, tracer, s_193_0);
        // S s_193_2: not s_193_1
        let s_193_2: bool = !s_193_1;
        // N s_193_3: branch s_193_2 b196 b194
        if s_193_2 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_194_0: const #0u : u8
        let s_194_0: bool = false;
        // D s_194_1: write-var gs#24261 <= s_194_0
        fn_state.gs_24261 = s_194_0;
        // N s_194_2: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_195_0: read-var gs#24261:u8
        let s_195_0: bool = fn_state.gs_24261;
        // D s_195_1: write-var route_to_el2 <= s_195_0
        fn_state.route_to_el2 = s_195_0;
        // N s_195_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_196_0: const #102552u : u32
        let s_196_0: u32 = 102552;
        // D s_196_1: read-reg s_196_0:struct
        let s_196_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_0 as isize);
            tracer.read_register(s_196_0 as isize, value);
            value
        };
        // D s_196_2: call _get_HCR_EL2_Type_TGE(s_196_1)
        let s_196_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_196_1);
        // C s_196_3: const #102552u : u32
        let s_196_3: u32 = 102552;
        // D s_196_4: read-reg s_196_3:struct
        let s_196_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_3 as isize);
            tracer.read_register(s_196_3 as isize, value);
            value
        };
        // D s_196_5: call _get_HCR_EL2_Type_AMO(s_196_4)
        let s_196_5: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_196_4);
        // D s_196_6: cast zx s_196_2 -> bv
        let s_196_6: Bits = Bits::new(s_196_2 as u128, 1u16);
        // D s_196_7: cast zx s_196_5 -> bv
        let s_196_7: Bits = Bits::new(s_196_5 as u128, 1u16);
        // D s_196_8: cast reint s_196_6 -> u128
        let s_196_8: u128 = (s_196_6.value() as u128);
        // D s_196_9: size-of s_196_6
        let s_196_9: u16 = s_196_6.length();
        // D s_196_10: cast reint s_196_7 -> u128
        let s_196_10: u128 = (s_196_7.value() as u128);
        // D s_196_11: size-of s_196_7
        let s_196_11: u16 = s_196_7.length();
        // D s_196_12: lsl s_196_8 s_196_11
        let s_196_12: u128 = s_196_8 << s_196_11;
        // D s_196_13: or s_196_12 s_196_10
        let s_196_13: u128 = ((s_196_12) | (s_196_10));
        // D s_196_14: add s_196_9 s_196_11
        let s_196_14: u16 = (s_196_9 + s_196_11);
        // D s_196_15: create-bits s_196_13 s_196_14
        let s_196_15: Bits = Bits::new(s_196_13, s_196_14);
        // D s_196_16: cast reint s_196_15 -> u8
        let s_196_16: u8 = (s_196_15.value() as u8);
        // D s_196_17: cast zx s_196_16 -> bv
        let s_196_17: Bits = Bits::new(s_196_16 as u128, 2u16);
        // C s_196_18: const #0u : u8
        let s_196_18: u8 = 0;
        // C s_196_19: cast zx s_196_18 -> bv
        let s_196_19: Bits = Bits::new(s_196_18 as u128, 2u16);
        // D s_196_20: cmp-ne s_196_17 s_196_19
        let s_196_20: bool = ((s_196_17) != (s_196_19));
        // D s_196_21: write-var gs#24261 <= s_196_20
        fn_state.gs_24261 = s_196_20;
        // N s_196_22: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_197_0: const #16975u : u32
        let s_197_0: u32 = 16975;
        // D s_197_1: read-reg s_197_0:u8
        let s_197_1: u8 = {
            let value = state.read_register::<u8>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: cast zx s_197_1 -> bv
        let s_197_2: Bits = Bits::new(s_197_1 as u128, 2u16);
        // C s_197_3: const #448u : u32
        let s_197_3: u32 = 448;
        // D s_197_4: read-reg s_197_3:u8
        let s_197_4: u8 = {
            let value = state.read_register::<u8>(s_197_3 as isize);
            tracer.read_register(s_197_3 as isize, value);
            value
        };
        // D s_197_5: cast zx s_197_4 -> bv
        let s_197_5: Bits = Bits::new(s_197_4 as u128, 2u16);
        // D s_197_6: cmp-eq s_197_2 s_197_5
        let s_197_6: bool = ((s_197_2) == (s_197_5));
        // D s_197_7: write-var gs#24260 <= s_197_6
        fn_state.gs_24260 = s_197_6;
        // N s_197_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_198_0: const #() : ()
        let s_198_0: () = ();
        // S s_198_1: call EL2Enabled(s_198_0)
        let s_198_1: bool = EL2Enabled(state, tracer, s_198_0);
        // D s_198_2: write-var gs#24259 <= s_198_1
        fn_state.gs_24259 = s_198_1;
        // N s_198_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_199_0: const #102552u : u32
        let s_199_0: u32 = 102552;
        // D s_199_1: read-reg s_199_0:struct
        let s_199_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_199_0 as isize);
            tracer.read_register(s_199_0 as isize, value);
            value
        };
        // D s_199_2: call _get_HCR_EL2_Type_AMO(s_199_1)
        let s_199_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_199_1);
        // D s_199_3: cast zx s_199_2 -> bv
        let s_199_3: Bits = Bits::new(s_199_2 as u128, 1u16);
        // C s_199_4: const #1u : u8
        let s_199_4: bool = true;
        // C s_199_5: cast zx s_199_4 -> bv
        let s_199_5: Bits = Bits::new(s_199_4 as u128, 1u16);
        // D s_199_6: cmp-eq s_199_3 s_199_5
        let s_199_6: bool = ((s_199_3) == (s_199_5));
        // D s_199_7: write-var route_to_el2 <= s_199_6
        fn_state.route_to_el2 = s_199_6;
        // N s_199_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_200_0: const #16975u : u32
        let s_200_0: u32 = 16975;
        // D s_200_1: read-reg s_200_0:u8
        let s_200_1: u8 = {
            let value = state.read_register::<u8>(s_200_0 as isize);
            tracer.read_register(s_200_0 as isize, value);
            value
        };
        // D s_200_2: cast zx s_200_1 -> bv
        let s_200_2: Bits = Bits::new(s_200_1 as u128, 2u16);
        // C s_200_3: const #440u : u32
        let s_200_3: u32 = 440;
        // D s_200_4: read-reg s_200_3:u8
        let s_200_4: u8 = {
            let value = state.read_register::<u8>(s_200_3 as isize);
            tracer.read_register(s_200_3 as isize, value);
            value
        };
        // D s_200_5: cast zx s_200_4 -> bv
        let s_200_5: Bits = Bits::new(s_200_4 as u128, 2u16);
        // D s_200_6: cmp-eq s_200_2 s_200_5
        let s_200_6: bool = ((s_200_2) == (s_200_5));
        // D s_200_7: write-var gs#24257 <= s_200_6
        fn_state.gs_24257 = s_200_6;
        // N s_200_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_201_0: const #() : ()
        let s_201_0: () = ();
        // S s_201_1: call EL2Enabled(s_201_0)
        let s_201_1: bool = EL2Enabled(state, tracer, s_201_0);
        // D s_201_2: write-var gs#24256 <= s_201_1
        fn_state.gs_24256 = s_201_1;
        // N s_201_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_202_0: const #424u : u32
        let s_202_0: u32 = 424;
        // D s_202_1: read-reg s_202_0:u8
        let s_202_1: u8 = {
            let value = state.read_register::<u8>(s_202_0 as isize);
            tracer.read_register(s_202_0 as isize, value);
            value
        };
        // C s_202_2: const #2u : u8
        let s_202_2: u8 = 2;
        // D s_202_3: cmp-lt s_202_1 s_202_2
        let s_202_3: bool = ((s_202_1) < (s_202_2));
        // N s_202_4: branch s_202_3 b205 b203
        if s_202_3 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_203_0: const #0u : u8
        let s_203_0: bool = false;
        // D s_203_1: write-var gs#24263 <= s_203_0
        fn_state.gs_24263 = s_203_0;
        // N s_203_2: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // D s_204_0: read-var gs#24263:u8
        let s_204_0: bool = fn_state.gs_24263;
        // D s_204_1: write-var route_to_el3 <= s_204_0
        fn_state.route_to_el3 = s_204_0;
        // N s_204_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea5cc8de4daab131c {
        // C s_205_0: const #() : ()
        let s_205_0: () = ();
        // S s_205_1: call EffectiveEA(s_205_0)
        let s_205_1: bool = EffectiveEA(state, tracer, s_205_0);
        // S s_205_2: cast zx s_205_1 -> bv
        let s_205_2: Bits = Bits::new(s_205_1 as u128, 1u16);
        // C s_205_3: const #1u : u8
        let s_205_3: bool = true;
        // C s_205_4: cast zx s_205_3 -> bv
        let s_205_4: Bits = Bits::new(s_205_3 as u128, 1u16);
        // S s_205_5: cmp-eq s_205_2 s_205_4
        let s_205_5: bool = ((s_205_2) == (s_205_4));
        // D s_205_6: write-var gs#24263 <= s_205_5
        fn_state.gs_24263 = s_205_5;
        // N s_205_7: jump b204
        return block_204(state, tracer, fn_state);
    }
}
