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
use u_get_HCR_EL2_Type_SWIO::*;
use DCInstNeedsTranslation::*;
use u__IMPDEF_integer::*;
use AArch32_Abort::*;
use HCR_read::*;
use Align_bits::*;
use VMID_read::*;
use AArch32_TranslateAddress::*;
use u__UNKNOWN_FullAddress::*;
use u_get_HCR_Type_SWIO::*;
use u_get_HCR_Type_VM::*;
use u_get_HCR_Type_DC::*;
use u_get_HCR_EL2_Type_DC::*;
use u_get_CTR_EL0_Type_DminLine::*;
use ELUsingAArch32::*;
use u__UNKNOWN_Shareability::*;
use CACHE_OP::*;
use CreateAccDescDC::*;
use SecurityStateAtEL::*;
use IsFault::*;
use integer_subrange::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_VM::*;
use ASID_read::*;
use DecodeSW::*;
use common::*;
pub fn AArch32_DC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regval: u32,
    cacheop: u32,
    opscope: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_23504: ProductTypef170cab34335b70c,
        gs_30309: bool,
        gs_30306: bool,
        gs_30277: bool,
        gs_30310: bool,
        gs_30279: bool,
        sizeshadow_559: i128,
        gs_30308: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        gs_30305: bool,
        gs_30307: bool,
        gs_30276: bool,
        gs_30293: bool,
        ga_23458: ProductTyped9cc76446c2fc207,
        cache: ProductType8ae001a7cc8b5154,
        gs_30278: bool,
        gs_30264: bool,
        size: i128,
        need_translate: bool,
        vaddress: u32,
        gs_30272: bool,
        gs_30274: bool,
        gs_30275: bool,
        gs_30273: bool,
        regval: u32,
        cacheop: u32,
        opscope: u32,
    }
    let fn_state = FunctionState {
        regval,
        cacheop,
        opscope,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #6u : u32
        let s_0_0: u32 = 6;
        // D s_0_1: write-var cache.0 <= s_0_0
        fn_state.cache._0 = s_0_0;
        // D s_0_2: read-var cacheop:u32
        let s_0_2: u32 = fn_state.cacheop;
        // D s_0_3: write-var cache.2 <= s_0_2
        fn_state.cache._2 = s_0_2;
        // D s_0_4: read-var opscope:u32
        let s_0_4: u32 = fn_state.opscope;
        // D s_0_5: write-var cache.8 <= s_0_4
        fn_state.cache._8 = s_0_4;
        // C s_0_6: const #0u : u32
        let s_0_6: u32 = 0;
        // D s_0_7: write-var cache.3 <= s_0_6
        fn_state.cache._3 = s_0_6;
        // C s_0_8: const #16975u : u32
        let s_0_8: u32 = 16975;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: call SecurityStateAtEL(s_0_9)
        let s_0_10: u32 = SecurityStateAtEL(state, tracer, s_0_9);
        // D s_0_11: write-var cache.11 <= s_0_10
        fn_state.cache._11 = s_0_10;
        // D s_0_12: read-var opscope:u32
        let s_0_12: u32 = fn_state.opscope;
        // C s_0_13: const #0u : u32
        let s_0_13: u32 = 0;
        // D s_0_14: cmp-eq s_0_12 s_0_13
        let s_0_14: bool = ((s_0_12) == (s_0_13));
        // N s_0_15: branch s_0_14 b49 b1
        if s_0_14 {
            return block_49(state, tracer, fn_state);
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
        // S s_1_1: call EL2Enabled(s_1_0)
        let s_1_1: bool = EL2Enabled(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b43 b2
        if s_1_1 {
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
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var cache.6 <= s_2_0
        fn_state.cache._6 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #448u : u32
        let s_3_3: u32 = 448;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // N s_3_7: branch s_3_6 b42 b4
        if s_3_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var cache.5 <= s_4_0
        fn_state.cache._5 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var opscope:u32
        let s_5_0: u32 = fn_state.opscope;
        // D s_5_1: call DCInstNeedsTranslation(s_5_0)
        let s_5_1: bool = DCInstNeedsTranslation(state, tracer, s_5_0);
        // D s_5_2: write-var need_translate <= s_5_1
        fn_state.need_translate = s_5_1;
        // D s_5_3: read-var regval:u32
        let s_5_3: u32 = fn_state.regval;
        // D s_5_4: write-var vaddress <= s_5_3
        fn_state.vaddress = s_5_3;
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // D s_5_6: write-var size <= s_5_5
        fn_state.size = s_5_5;
        // D s_5_7: read-var cacheop:u32
        let s_5_7: u32 = fn_state.cacheop;
        // C s_5_8: const #1u : u32
        let s_5_8: u32 = 1;
        // D s_5_9: cmp-eq s_5_7 s_5_8
        let s_5_9: bool = ((s_5_7) == (s_5_8));
        // N s_5_10: branch s_5_9 b38 b6
        if s_5_9 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var size:i
        let s_7_0: i128 = fn_state.size;
        // D s_7_1: write-var sizeshadow#559 <= s_7_0
        fn_state.sizeshadow_559 = s_7_0;
        // D s_7_2: read-var need_translate:u8
        let s_7_2: bool = fn_state.need_translate;
        // D s_7_3: write-var cache.14 <= s_7_2
        fn_state.cache._14 = s_7_2;
        // C s_7_4: const #64s : i
        let s_7_4: i128 = 64;
        // D s_7_5: read-var vaddress:u32
        let s_7_5: u32 = fn_state.vaddress;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 32u16);
        // D s_7_7: bits-cast zx s_7_6 -> bv length s_7_4
        let s_7_7: Bits = s_7_6.zero_extend(s_7_4);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // D s_7_9: write-var cache.15 <= s_7_8
        fn_state.cache._15 = s_7_8;
        // D s_7_10: read-var need_translate:u8
        let s_7_10: bool = fn_state.need_translate;
        // N s_7_11: branch s_7_10 b31 b8
        if s_7_10 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call __UNKNOWN_Shareability(s_8_0)
        let s_8_1: u32 = u__UNKNOWN_Shareability(state, tracer, s_8_0);
        // D s_8_2: write-var cache.13 <= s_8_1
        fn_state.cache._13 = s_8_1;
        // C s_8_3: const #() : ()
        let s_8_3: () = ();
        // S s_8_4: call __UNKNOWN_FullAddress(s_8_3)
        let s_8_4: ProductTypeda0231e9dc169f81 = u__UNKNOWN_FullAddress(
            state,
            tracer,
            s_8_3,
        );
        // D s_8_5: write-var cache.9 <= s_8_4
        fn_state.cache._9 = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var cacheop:u32
        let s_9_0: u32 = fn_state.cacheop;
        // C s_9_1: const #1u : u32
        let s_9_1: u32 = 1;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b30 b10
        if s_9_2 {
            return block_30(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#30305 <= s_10_0
        fn_state.gs_30305 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#30305:u8
        let s_11_0: bool = fn_state.gs_30305;
        // N s_11_1: branch s_11_0 b29 b12
        if s_11_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#30306 <= s_12_0
        fn_state.gs_30306 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#30306:u8
        let s_13_0: bool = fn_state.gs_30306;
        // N s_13_1: branch s_13_0 b19 b14
        if s_13_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#30310 <= s_14_0
        fn_state.gs_30310 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#30310:u8
        let s_15_0: bool = fn_state.gs_30310;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
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
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var cache:struct
        let s_17_0: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_17_1: call CACHE_OP(s_17_0)
        let s_17_1: () = CACHE_OP(state, tracer, s_17_0);
        // N s_17_2: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: write-var cache.2 <= s_18_0
        fn_state.cache._2 = s_18_0;
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #432u : u32
        let s_19_0: u32 = 432;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call ELUsingAArch32(s_19_1)
        let s_19_2: bool = ELUsingAArch32(state, tracer, s_19_1);
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // N s_19_4: branch s_19_3 b28 b20
        if s_19_3 {
            return block_28(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#30307 <= s_20_0
        fn_state.gs_30307 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#30307:u8
        let s_21_0: bool = fn_state.gs_30307;
        // N s_21_1: branch s_21_0 b27 b22
        if s_21_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #432u : u32
        let s_22_0: u32 = 432;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call ELUsingAArch32(s_22_1)
        let s_22_2: bool = ELUsingAArch32(state, tracer, s_22_1);
        // N s_22_3: branch s_22_2 b26 b23
        if s_22_2 {
            return block_26(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#30308 <= s_23_0
        fn_state.gs_30308 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#30308:u8
        let s_24_0: bool = fn_state.gs_30308;
        // D s_24_1: write-var gs#30309 <= s_24_0
        fn_state.gs_30309 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#30309:u8
        let s_25_0: bool = fn_state.gs_30309;
        // D s_25_1: write-var gs#30310 <= s_25_0
        fn_state.gs_30310 = s_25_0;
        // N s_25_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call HCR_read(s_26_0)
        let s_26_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_26_0);
        // S s_26_2: call _get_HCR_Type_DC(s_26_1)
        let s_26_2: bool = u_get_HCR_Type_DC(state, tracer, s_26_1);
        // C s_26_3: const #() : ()
        let s_26_3: () = ();
        // S s_26_4: call HCR_read(s_26_3)
        let s_26_4: ProductType700c18a878c5601b = HCR_read(state, tracer, s_26_3);
        // S s_26_5: call _get_HCR_Type_VM(s_26_4)
        let s_26_5: bool = u_get_HCR_Type_VM(state, tracer, s_26_4);
        // S s_26_6: cast zx s_26_2 -> bv
        let s_26_6: Bits = Bits::new(s_26_2 as u128, 1u16);
        // S s_26_7: cast zx s_26_5 -> bv
        let s_26_7: Bits = Bits::new(s_26_5 as u128, 1u16);
        // S s_26_8: cast reint s_26_6 -> u128
        let s_26_8: u128 = (s_26_6.value() as u128);
        // D s_26_9: size-of s_26_6
        let s_26_9: u16 = s_26_6.length();
        // S s_26_10: cast reint s_26_7 -> u128
        let s_26_10: u128 = (s_26_7.value() as u128);
        // D s_26_11: size-of s_26_7
        let s_26_11: u16 = s_26_7.length();
        // D s_26_12: lsl s_26_8 s_26_11
        let s_26_12: u128 = s_26_8 << s_26_11;
        // D s_26_13: or s_26_12 s_26_10
        let s_26_13: u128 = ((s_26_12) | (s_26_10));
        // D s_26_14: add s_26_9 s_26_11
        let s_26_14: u16 = (s_26_9 + s_26_11);
        // D s_26_15: create-bits s_26_13 s_26_14
        let s_26_15: Bits = Bits::new(s_26_13, s_26_14);
        // D s_26_16: cast reint s_26_15 -> u8
        let s_26_16: u8 = (s_26_15.value() as u8);
        // D s_26_17: cast zx s_26_16 -> bv
        let s_26_17: Bits = Bits::new(s_26_16 as u128, 2u16);
        // C s_26_18: const #0u : u8
        let s_26_18: u8 = 0;
        // C s_26_19: cast zx s_26_18 -> bv
        let s_26_19: Bits = Bits::new(s_26_18 as u128, 2u16);
        // D s_26_20: cmp-ne s_26_17 s_26_19
        let s_26_20: bool = ((s_26_17) != (s_26_19));
        // D s_26_21: write-var gs#30308 <= s_26_20
        fn_state.gs_30308 = s_26_20;
        // N s_26_22: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#30309 <= s_27_0
        fn_state.gs_30309 = s_27_0;
        // N s_27_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #102552u : u32
        let s_28_0: u32 = 102552;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_HCR_EL2_Type_DC(s_28_1)
        let s_28_2: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_28_1);
        // C s_28_3: const #102552u : u32
        let s_28_3: u32 = 102552;
        // D s_28_4: read-reg s_28_3:struct
        let s_28_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_3 as isize);
            tracer.read_register(s_28_3 as isize, value);
            value
        };
        // D s_28_5: call _get_HCR_EL2_Type_VM(s_28_4)
        let s_28_5: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_28_4);
        // D s_28_6: cast zx s_28_2 -> bv
        let s_28_6: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_7: cast zx s_28_5 -> bv
        let s_28_7: Bits = Bits::new(s_28_5 as u128, 1u16);
        // D s_28_8: cast reint s_28_6 -> u128
        let s_28_8: u128 = (s_28_6.value() as u128);
        // D s_28_9: size-of s_28_6
        let s_28_9: u16 = s_28_6.length();
        // D s_28_10: cast reint s_28_7 -> u128
        let s_28_10: u128 = (s_28_7.value() as u128);
        // D s_28_11: size-of s_28_7
        let s_28_11: u16 = s_28_7.length();
        // D s_28_12: lsl s_28_8 s_28_11
        let s_28_12: u128 = s_28_8 << s_28_11;
        // D s_28_13: or s_28_12 s_28_10
        let s_28_13: u128 = ((s_28_12) | (s_28_10));
        // D s_28_14: add s_28_9 s_28_11
        let s_28_14: u16 = (s_28_9 + s_28_11);
        // D s_28_15: create-bits s_28_13 s_28_14
        let s_28_15: Bits = Bits::new(s_28_13, s_28_14);
        // D s_28_16: cast reint s_28_15 -> u8
        let s_28_16: u8 = (s_28_15.value() as u8);
        // D s_28_17: cast zx s_28_16 -> bv
        let s_28_17: Bits = Bits::new(s_28_16 as u128, 2u16);
        // C s_28_18: const #0u : u8
        let s_28_18: u8 = 0;
        // C s_28_19: cast zx s_28_18 -> bv
        let s_28_19: Bits = Bits::new(s_28_18 as u128, 2u16);
        // D s_28_20: cmp-ne s_28_17 s_28_19
        let s_28_20: bool = ((s_28_17) != (s_28_19));
        // D s_28_21: write-var gs#30307 <= s_28_20
        fn_state.gs_30307 = s_28_20;
        // N s_28_22: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EL2Enabled(s_29_0)
        let s_29_1: bool = EL2Enabled(state, tracer, s_29_0);
        // D s_29_2: write-var gs#30306 <= s_29_1
        fn_state.gs_30306 = s_29_1;
        // N s_29_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #16975u : u32
        let s_30_0: u32 = 16975;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: cast zx s_30_1 -> bv
        let s_30_2: Bits = Bits::new(s_30_1 as u128, 2u16);
        // C s_30_3: const #440u : u32
        let s_30_3: u32 = 440;
        // D s_30_4: read-reg s_30_3:u8
        let s_30_4: u8 = {
            let value = state.read_register::<u8>(s_30_3 as isize);
            tracer.read_register(s_30_3 as isize, value);
            value
        };
        // D s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 2u16);
        // D s_30_6: cmp-eq s_30_2 s_30_5
        let s_30_6: bool = ((s_30_2) == (s_30_5));
        // D s_30_7: write-var gs#30305 <= s_30_6
        fn_state.gs_30305 = s_30_6;
        // N s_30_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: read-var cache:struct
        let s_31_1: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_31_2: call CreateAccDescDC(s_31_1)
        let s_31_2: ProductType9878976b5bcce9c9 = CreateAccDescDC(state, tracer, s_31_1);
        // D s_31_3: read-var vaddress:u32
        let s_31_3: u32 = fn_state.vaddress;
        // D s_31_4: read-var sizeshadow#559:i
        let s_31_4: i128 = fn_state.sizeshadow_559;
        // D s_31_5: call AArch32_TranslateAddress(s_31_3, s_31_2, s_31_0, s_31_4)
        let s_31_5: ProductTypece7c66ccb2cab13e = AArch32_TranslateAddress(
            state,
            tracer,
            s_31_3,
            s_31_2,
            s_31_0,
            s_31_4,
        );
        // D s_31_6: write-var memaddrdesc <= s_31_5
        fn_state.memaddrdesc = s_31_5;
        // N s_31_7: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var memaddrdesc:struct
        let s_32_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_32_1: call IsFault(s_32_0)
        let s_32_1: bool = IsFault(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b37 b33
        if s_32_1 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var memaddrdesc.3:struct
        let s_34_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_34_1: write-var cache.9 <= s_34_0
        fn_state.cache._9 = s_34_0;
        // D s_34_2: read-var opscope:u32
        let s_34_2: u32 = fn_state.opscope;
        // C s_34_3: const #2u : u32
        let s_34_3: u32 = 2;
        // D s_34_4: cmp-eq s_34_2 s_34_3
        let s_34_4: bool = ((s_34_2) == (s_34_3));
        // N s_34_5: branch s_34_4 b36 b35
        if s_34_4 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u32
        let s_35_0: u32 = 0;
        // D s_35_1: write-var cache.13 <= s_35_0
        fn_state.cache._13 = s_35_0;
        // N s_35_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var memaddrdesc.2:struct
        let s_36_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_36_1: write-var ga#23504 <= s_36_0
        fn_state.ga_23504 = s_36_0;
        // D s_36_2: read-var ga#23504.5:struct
        let s_36_2: u32 = fn_state.ga_23504._5;
        // D s_36_3: write-var cache.13 <= s_36_2
        fn_state.cache._13 = s_36_2;
        // N s_36_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var memaddrdesc.0:struct
        let s_37_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_37_1: read-var regval:u32
        let s_37_1: u32 = fn_state.regval;
        // D s_37_2: call AArch32_Abort(s_37_1, s_37_0)
        let s_37_2: () = AArch32_Abort(state, tracer, s_37_1, s_37_0);
        // N s_37_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #"Data Cache Invalidate Watchpoint Size" : str
        let s_38_0: &'static str = "Data Cache Invalidate Watchpoint Size";
        // S s_38_1: call __IMPDEF_integer(s_38_0)
        let s_38_1: i128 = u__IMPDEF_integer(state, tracer, s_38_0);
        // D s_38_2: write-var size <= s_38_1
        fn_state.size = s_38_1;
        // C s_38_3: const #19056u : u32
        let s_38_3: u32 = 19056;
        // D s_38_4: read-reg s_38_3:struct
        let s_38_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_3 as isize);
            tracer.read_register(s_38_3 as isize, value);
            value
        };
        // D s_38_5: call _get_CTR_EL0_Type_DminLine(s_38_4)
        let s_38_5: u8 = u_get_CTR_EL0_Type_DminLine(state, tracer, s_38_4);
        // D s_38_6: cast zx s_38_5 -> bv
        let s_38_6: Bits = Bits::new(s_38_5 as u128, 4u16);
        // D s_38_7: cast zx s_38_6 -> i
        let s_38_7: i128 = (s_38_6.value() as i128);
        // D s_38_8: cast reint s_38_7 -> i64
        let s_38_8: i64 = (s_38_7 as i64);
        // D s_38_9: cast zx s_38_8 -> i
        let s_38_9: i128 = (i128::try_from(s_38_8).unwrap());
        // D s_38_10: pow2 s_38_9
        let s_38_10: i128 = (s_38_9).pow(2);
        // D s_38_11: cast reint s_38_10 -> i64
        let s_38_11: i64 = (s_38_10 as i64);
        // C s_38_12: const #4s : i
        let s_38_12: i128 = 4;
        // D s_38_13: cast zx s_38_11 -> i
        let s_38_13: i128 = (i128::try_from(s_38_11).unwrap());
        // D s_38_14: mul s_38_12 s_38_13
        let s_38_14: i128 = ((s_38_12) * (s_38_13));
        // D s_38_15: cast reint s_38_14 -> i64
        let s_38_15: i64 = (s_38_14 as i64);
        // D s_38_16: cast zx s_38_15 -> i
        let s_38_16: i128 = (i128::try_from(s_38_15).unwrap());
        // D s_38_17: read-var size:i
        let s_38_17: i128 = fn_state.size;
        // D s_38_18: cmp-ge s_38_17 s_38_16
        let s_38_18: bool = ((s_38_17) >= (s_38_16));
        // N s_38_19: branch s_38_18 b41 b39
        if s_38_18 {
            return block_41(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#30293 <= s_39_0
        fn_state.gs_30293 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#30293:u8
        let s_40_0: bool = fn_state.gs_30293;
        // N s_40_1: assert s_40_0
        let s_40_1: () = assert!(s_40_0);
        // C s_40_2: const #32s : i
        let s_40_2: i128 = 32;
        // C s_40_3: const #0s : i
        let s_40_3: i128 = 0;
        // D s_40_4: read-var size:i
        let s_40_4: i128 = fn_state.size;
        // D s_40_5: call integer_subrange(s_40_4, s_40_2, s_40_3)
        let s_40_5: Bits = integer_subrange(state, tracer, s_40_4, s_40_2, s_40_3);
        // D s_40_6: cast reint s_40_5 -> u33
        let s_40_6: u64 = (s_40_5.value() as u64);
        // C s_40_7: const #1s : i
        let s_40_7: i128 = 1;
        // D s_40_8: read-var size:i
        let s_40_8: i128 = fn_state.size;
        // D s_40_9: sub s_40_8 s_40_7
        let s_40_9: i128 = ((s_40_8) - (s_40_7));
        // C s_40_10: const #32s : i
        let s_40_10: i128 = 32;
        // C s_40_11: const #0s : i
        let s_40_11: i128 = 0;
        // D s_40_12: call integer_subrange(s_40_9, s_40_10, s_40_11)
        let s_40_12: Bits = integer_subrange(state, tracer, s_40_9, s_40_10, s_40_11);
        // D s_40_13: cast reint s_40_12 -> u33
        let s_40_13: u64 = (s_40_12.value() as u64);
        // D s_40_14: cast zx s_40_6 -> bv
        let s_40_14: Bits = Bits::new(s_40_6 as u128, 33u16);
        // D s_40_15: cast zx s_40_13 -> bv
        let s_40_15: Bits = Bits::new(s_40_13 as u128, 33u16);
        // D s_40_16: and s_40_14 s_40_15
        let s_40_16: Bits = ((s_40_14) & (s_40_15));
        // D s_40_17: cast reint s_40_16 -> u33
        let s_40_17: u64 = (s_40_16.value() as u64);
        // D s_40_18: cast zx s_40_17 -> bv
        let s_40_18: Bits = Bits::new(s_40_17 as u128, 33u16);
        // D s_40_19: cast zx s_40_18 -> i
        let s_40_19: i128 = (s_40_18.value() as i128);
        // D s_40_20: cast reint s_40_19 -> i64
        let s_40_20: i64 = (s_40_19 as i64);
        // C s_40_21: const #0s : i
        let s_40_21: i128 = 0;
        // D s_40_22: cast zx s_40_20 -> i
        let s_40_22: i128 = (i128::try_from(s_40_20).unwrap());
        // D s_40_23: cmp-eq s_40_22 s_40_21
        let s_40_23: bool = ((s_40_22) == (s_40_21));
        // N s_40_24: assert s_40_23
        let s_40_24: () = assert!(s_40_23);
        // D s_40_25: read-var regval:u32
        let s_40_25: u32 = fn_state.regval;
        // D s_40_26: cast zx s_40_25 -> bv
        let s_40_26: Bits = Bits::new(s_40_25 as u128, 32u16);
        // D s_40_27: read-var size:i
        let s_40_27: i128 = fn_state.size;
        // D s_40_28: call Align_bits(s_40_26, s_40_27)
        let s_40_28: Bits = Align_bits(state, tracer, s_40_26, s_40_27);
        // D s_40_29: cast reint s_40_28 -> u32
        let s_40_29: u32 = (s_40_28.value() as u32);
        // D s_40_30: write-var vaddress <= s_40_29
        fn_state.vaddress = s_40_29;
        // N s_40_31: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #2048s : i
        let s_41_0: i128 = 2048;
        // D s_41_1: read-var size:i
        let s_41_1: i128 = fn_state.size;
        // D s_41_2: cmp-le s_41_1 s_41_0
        let s_41_2: bool = ((s_41_1) <= (s_41_0));
        // D s_41_3: write-var gs#30293 <= s_41_2
        fn_state.gs_30293 = s_41_2;
        // N s_41_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var cache.5 <= s_42_0
        fn_state.cache._5 = s_42_0;
        // C s_42_2: const #() : ()
        let s_42_2: () = ();
        // S s_42_3: call ASID_read(s_42_2)
        let s_42_3: u16 = ASID_read(state, tracer, s_42_2);
        // D s_42_4: write-var cache.1 <= s_42_3
        fn_state.cache._1 = s_42_3;
        // N s_42_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #16975u : u32
        let s_43_0: u32 = 16975;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 2u16);
        // C s_43_3: const #448u : u32
        let s_43_3: u32 = 448;
        // D s_43_4: read-reg s_43_3:u8
        let s_43_4: u8 = {
            let value = state.read_register::<u8>(s_43_3 as isize);
            tracer.read_register(s_43_3 as isize, value);
            value
        };
        // D s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 2u16);
        // D s_43_6: cmp-eq s_43_2 s_43_5
        let s_43_6: bool = ((s_43_2) == (s_43_5));
        // N s_43_7: branch s_43_6 b48 b44
        if s_43_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #16975u : u32
        let s_44_0: u32 = 16975;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: cast zx s_44_1 -> bv
        let s_44_2: Bits = Bits::new(s_44_1 as u128, 2u16);
        // C s_44_3: const #440u : u32
        let s_44_3: u32 = 440;
        // D s_44_4: read-reg s_44_3:u8
        let s_44_4: u8 = {
            let value = state.read_register::<u8>(s_44_3 as isize);
            tracer.read_register(s_44_3 as isize, value);
            value
        };
        // D s_44_5: cast zx s_44_4 -> bv
        let s_44_5: Bits = Bits::new(s_44_4 as u128, 2u16);
        // D s_44_6: cmp-eq s_44_2 s_44_5
        let s_44_6: bool = ((s_44_2) == (s_44_5));
        // D s_44_7: write-var gs#30264 <= s_44_6
        fn_state.gs_30264 = s_44_6;
        // N s_44_8: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#30264:u8
        let s_45_0: bool = fn_state.gs_30264;
        // N s_45_1: branch s_45_0 b47 b46
        if s_45_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_46_1: write-var cache.6 <= s_46_0
        fn_state.cache._6 = s_46_0;
        // N s_46_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var cache.6 <= s_47_0
        fn_state.cache._6 = s_47_0;
        // C s_47_2: const #() : ()
        let s_47_2: () = ();
        // S s_47_3: call VMID_read(s_47_2)
        let s_47_3: u16 = VMID_read(state, tracer, s_47_2);
        // D s_47_4: write-var cache.16 <= s_47_3
        fn_state.cache._16 = s_47_3;
        // N s_47_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#30264 <= s_48_0
        fn_state.gs_30264 = s_48_0;
        // N s_48_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u32
        let s_49_0: u32 = 0;
        // D s_49_1: write-var cache.13 <= s_49_0
        fn_state.cache._13 = s_49_0;
        // C s_49_2: const #64s : i
        let s_49_2: i128 = 64;
        // D s_49_3: read-var regval:u32
        let s_49_3: u32 = fn_state.regval;
        // D s_49_4: cast zx s_49_3 -> bv
        let s_49_4: Bits = Bits::new(s_49_3 as u128, 32u16);
        // D s_49_5: bits-cast zx s_49_4 -> bv length s_49_2
        let s_49_5: Bits = s_49_4.zero_extend(s_49_2);
        // D s_49_6: cast reint s_49_5 -> u64
        let s_49_6: u64 = (s_49_5.value() as u64);
        // C s_49_7: const #0u : u32
        let s_49_7: u32 = 0;
        // D s_49_8: call DecodeSW(s_49_6, s_49_7)
        let s_49_8: ProductTyped9cc76446c2fc207 = DecodeSW(
            state,
            tracer,
            s_49_6,
            s_49_7,
        );
        // D s_49_9: write-var ga#23458 <= s_49_8
        fn_state.ga_23458 = s_49_8;
        // D s_49_10: read-var ga#23458.0:struct
        let s_49_10: i128 = fn_state.ga_23458._0;
        // D s_49_11: read-var ga#23458.1:struct
        let s_49_11: i128 = fn_state.ga_23458._1;
        // D s_49_12: read-var ga#23458.2:struct
        let s_49_12: i128 = fn_state.ga_23458._2;
        // D s_49_13: write-var cache.12 <= s_49_10
        fn_state.cache._12 = s_49_10;
        // D s_49_14: write-var cache.17 <= s_49_11
        fn_state.cache._17 = s_49_11;
        // D s_49_15: write-var cache.7 <= s_49_12
        fn_state.cache._7 = s_49_12;
        // D s_49_16: read-var cacheop:u32
        let s_49_16: u32 = fn_state.cacheop;
        // C s_49_17: const #1u : u32
        let s_49_17: u32 = 1;
        // D s_49_18: cmp-eq s_49_16 s_49_17
        let s_49_18: bool = ((s_49_16) == (s_49_17));
        // N s_49_19: branch s_49_18 b76 b50
        if s_49_18 {
            return block_76(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#30272 <= s_50_0
        fn_state.gs_30272 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#30272:u8
        let s_51_0: bool = fn_state.gs_30272;
        // N s_51_1: branch s_51_0 b75 b52
        if s_51_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#30273 <= s_52_0
        fn_state.gs_30273 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#30273:u8
        let s_53_0: bool = fn_state.gs_30273;
        // N s_53_1: branch s_53_0 b59 b54
        if s_53_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#30279 <= s_54_0
        fn_state.gs_30279 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#30279:u8
        let s_55_0: bool = fn_state.gs_30279;
        // N s_55_1: branch s_55_0 b58 b56
        if s_55_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var cache:struct
        let s_57_0: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_57_1: call CACHE_OP(s_57_0)
        let s_57_1: () = CACHE_OP(state, tracer, s_57_0);
        // N s_57_2: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #2u : u32
        let s_58_0: u32 = 2;
        // D s_58_1: write-var cache.2 <= s_58_0
        fn_state.cache._2 = s_58_0;
        // N s_58_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #432u : u32
        let s_59_0: u32 = 432;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call ELUsingAArch32(s_59_1)
        let s_59_2: bool = ELUsingAArch32(state, tracer, s_59_1);
        // D s_59_3: not s_59_2
        let s_59_3: bool = !s_59_2;
        // N s_59_4: branch s_59_3 b71 b60
        if s_59_3 {
            return block_71(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#30275 <= s_60_0
        fn_state.gs_30275 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#30275:u8
        let s_61_0: bool = fn_state.gs_30275;
        // N s_61_1: branch s_61_0 b70 b62
        if s_61_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #432u : u32
        let s_62_0: u32 = 432;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call ELUsingAArch32(s_62_1)
        let s_62_2: bool = ELUsingAArch32(state, tracer, s_62_1);
        // N s_62_3: branch s_62_2 b66 b63
        if s_62_2 {
            return block_66(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#30277 <= s_63_0
        fn_state.gs_30277 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#30277:u8
        let s_64_0: bool = fn_state.gs_30277;
        // D s_64_1: write-var gs#30278 <= s_64_0
        fn_state.gs_30278 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#30278:u8
        let s_65_0: bool = fn_state.gs_30278;
        // D s_65_1: write-var gs#30279 <= s_65_0
        fn_state.gs_30279 = s_65_0;
        // N s_65_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call HCR_read(s_66_0)
        let s_66_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_66_0);
        // S s_66_2: call _get_HCR_Type_SWIO(s_66_1)
        let s_66_2: bool = u_get_HCR_Type_SWIO(state, tracer, s_66_1);
        // S s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // C s_66_4: const #1u : u8
        let s_66_4: bool = true;
        // C s_66_5: cast zx s_66_4 -> bv
        let s_66_5: Bits = Bits::new(s_66_4 as u128, 1u16);
        // S s_66_6: cmp-eq s_66_3 s_66_5
        let s_66_6: bool = ((s_66_3) == (s_66_5));
        // N s_66_7: branch s_66_6 b69 b67
        if s_66_6 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call HCR_read(s_67_0)
        let s_67_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_67_0);
        // S s_67_2: call _get_HCR_Type_DC(s_67_1)
        let s_67_2: bool = u_get_HCR_Type_DC(state, tracer, s_67_1);
        // C s_67_3: const #() : ()
        let s_67_3: () = ();
        // S s_67_4: call HCR_read(s_67_3)
        let s_67_4: ProductType700c18a878c5601b = HCR_read(state, tracer, s_67_3);
        // S s_67_5: call _get_HCR_Type_VM(s_67_4)
        let s_67_5: bool = u_get_HCR_Type_VM(state, tracer, s_67_4);
        // S s_67_6: cast zx s_67_2 -> bv
        let s_67_6: Bits = Bits::new(s_67_2 as u128, 1u16);
        // S s_67_7: cast zx s_67_5 -> bv
        let s_67_7: Bits = Bits::new(s_67_5 as u128, 1u16);
        // S s_67_8: cast reint s_67_6 -> u128
        let s_67_8: u128 = (s_67_6.value() as u128);
        // D s_67_9: size-of s_67_6
        let s_67_9: u16 = s_67_6.length();
        // S s_67_10: cast reint s_67_7 -> u128
        let s_67_10: u128 = (s_67_7.value() as u128);
        // D s_67_11: size-of s_67_7
        let s_67_11: u16 = s_67_7.length();
        // D s_67_12: lsl s_67_8 s_67_11
        let s_67_12: u128 = s_67_8 << s_67_11;
        // D s_67_13: or s_67_12 s_67_10
        let s_67_13: u128 = ((s_67_12) | (s_67_10));
        // D s_67_14: add s_67_9 s_67_11
        let s_67_14: u16 = (s_67_9 + s_67_11);
        // D s_67_15: create-bits s_67_13 s_67_14
        let s_67_15: Bits = Bits::new(s_67_13, s_67_14);
        // D s_67_16: cast reint s_67_15 -> u8
        let s_67_16: u8 = (s_67_15.value() as u8);
        // D s_67_17: cast zx s_67_16 -> bv
        let s_67_17: Bits = Bits::new(s_67_16 as u128, 2u16);
        // C s_67_18: const #0u : u8
        let s_67_18: u8 = 0;
        // C s_67_19: cast zx s_67_18 -> bv
        let s_67_19: Bits = Bits::new(s_67_18 as u128, 2u16);
        // D s_67_20: cmp-ne s_67_17 s_67_19
        let s_67_20: bool = ((s_67_17) != (s_67_19));
        // D s_67_21: write-var gs#30276 <= s_67_20
        fn_state.gs_30276 = s_67_20;
        // N s_67_22: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#30276:u8
        let s_68_0: bool = fn_state.gs_30276;
        // D s_68_1: write-var gs#30277 <= s_68_0
        fn_state.gs_30277 = s_68_0;
        // N s_68_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#30276 <= s_69_0
        fn_state.gs_30276 = s_69_0;
        // N s_69_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#30278 <= s_70_0
        fn_state.gs_30278 = s_70_0;
        // N s_70_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #102552u : u32
        let s_71_0: u32 = 102552;
        // D s_71_1: read-reg s_71_0:struct
        let s_71_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // D s_71_2: call _get_HCR_EL2_Type_SWIO(s_71_1)
        let s_71_2: bool = u_get_HCR_EL2_Type_SWIO(state, tracer, s_71_1);
        // D s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // C s_71_4: const #1u : u8
        let s_71_4: bool = true;
        // C s_71_5: cast zx s_71_4 -> bv
        let s_71_5: Bits = Bits::new(s_71_4 as u128, 1u16);
        // D s_71_6: cmp-eq s_71_3 s_71_5
        let s_71_6: bool = ((s_71_3) == (s_71_5));
        // N s_71_7: branch s_71_6 b74 b72
        if s_71_6 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #102552u : u32
        let s_72_0: u32 = 102552;
        // D s_72_1: read-reg s_72_0:struct
        let s_72_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: call _get_HCR_EL2_Type_DC(s_72_1)
        let s_72_2: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_72_1);
        // C s_72_3: const #102552u : u32
        let s_72_3: u32 = 102552;
        // D s_72_4: read-reg s_72_3:struct
        let s_72_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_3 as isize);
            tracer.read_register(s_72_3 as isize, value);
            value
        };
        // D s_72_5: call _get_HCR_EL2_Type_VM(s_72_4)
        let s_72_5: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_72_4);
        // D s_72_6: cast zx s_72_2 -> bv
        let s_72_6: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_7: cast zx s_72_5 -> bv
        let s_72_7: Bits = Bits::new(s_72_5 as u128, 1u16);
        // D s_72_8: cast reint s_72_6 -> u128
        let s_72_8: u128 = (s_72_6.value() as u128);
        // D s_72_9: size-of s_72_6
        let s_72_9: u16 = s_72_6.length();
        // D s_72_10: cast reint s_72_7 -> u128
        let s_72_10: u128 = (s_72_7.value() as u128);
        // D s_72_11: size-of s_72_7
        let s_72_11: u16 = s_72_7.length();
        // D s_72_12: lsl s_72_8 s_72_11
        let s_72_12: u128 = s_72_8 << s_72_11;
        // D s_72_13: or s_72_12 s_72_10
        let s_72_13: u128 = ((s_72_12) | (s_72_10));
        // D s_72_14: add s_72_9 s_72_11
        let s_72_14: u16 = (s_72_9 + s_72_11);
        // D s_72_15: create-bits s_72_13 s_72_14
        let s_72_15: Bits = Bits::new(s_72_13, s_72_14);
        // D s_72_16: cast reint s_72_15 -> u8
        let s_72_16: u8 = (s_72_15.value() as u8);
        // D s_72_17: cast zx s_72_16 -> bv
        let s_72_17: Bits = Bits::new(s_72_16 as u128, 2u16);
        // C s_72_18: const #0u : u8
        let s_72_18: u8 = 0;
        // C s_72_19: cast zx s_72_18 -> bv
        let s_72_19: Bits = Bits::new(s_72_18 as u128, 2u16);
        // D s_72_20: cmp-ne s_72_17 s_72_19
        let s_72_20: bool = ((s_72_17) != (s_72_19));
        // D s_72_21: write-var gs#30274 <= s_72_20
        fn_state.gs_30274 = s_72_20;
        // N s_72_22: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#30274:u8
        let s_73_0: bool = fn_state.gs_30274;
        // D s_73_1: write-var gs#30275 <= s_73_0
        fn_state.gs_30275 = s_73_0;
        // N s_73_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#30274 <= s_74_0
        fn_state.gs_30274 = s_74_0;
        // N s_74_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call EL2Enabled(s_75_0)
        let s_75_1: bool = EL2Enabled(state, tracer, s_75_0);
        // D s_75_2: write-var gs#30273 <= s_75_1
        fn_state.gs_30273 = s_75_1;
        // N s_75_3: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #16975u : u32
        let s_76_0: u32 = 16975;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: cast zx s_76_1 -> bv
        let s_76_2: Bits = Bits::new(s_76_1 as u128, 2u16);
        // C s_76_3: const #440u : u32
        let s_76_3: u32 = 440;
        // D s_76_4: read-reg s_76_3:u8
        let s_76_4: u8 = {
            let value = state.read_register::<u8>(s_76_3 as isize);
            tracer.read_register(s_76_3 as isize, value);
            value
        };
        // D s_76_5: cast zx s_76_4 -> bv
        let s_76_5: Bits = Bits::new(s_76_4 as u128, 2u16);
        // D s_76_6: cmp-eq s_76_2 s_76_5
        let s_76_6: bool = ((s_76_2) == (s_76_5));
        // D s_76_7: write-var gs#30272 <= s_76_6
        fn_state.gs_30272 = s_76_6;
        // N s_76_8: jump b51
        return block_51(state, tracer, fn_state);
    }
}
