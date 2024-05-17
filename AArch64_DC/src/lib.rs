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
use IsInHost::*;
use DCInstNeedsTranslation::*;
use DecodePASpace::*;
use VMID_read::*;
use u__UNKNOWN_FullAddress::*;
use AArch64_TranslateAddress::*;
use GranuleProtectionCheck::*;
use u__UNKNOWN_bits::*;
use EndOfInstruction::*;
use u__UNKNOWN_Shareability::*;
use CACHE_OP::*;
use CPASAtPAS::*;
use SecurityStateAtEL::*;
use AArch64_Abort::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_VM::*;
use u_get_HCR_EL2_Type_SWIO::*;
use u__IMPDEF_integer::*;
use Align_bits::*;
use u__IMPDEF_boolean::*;
use u_get_HCR_EL2_Type_DC::*;
use u_get_CTR_EL0_Type_DminLine::*;
use ASID_read::*;
use DecodeSW::*;
use CreateAccDescDC::*;
use integer_subrange::*;
use CPASAtSecurityState::*;
use HaveSecureState::*;
use IsFault::*;
use common::*;
pub fn AArch64_DC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regval: u64,
    cachetype: u32,
    cacheop: u32,
    opscope_in: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_27258: bool,
        gs_27274: bool,
        gs_27271: bool,
        ga_21012: ProductType396b95aa74979079,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        gs_27346: bool,
        gs_27288: bool,
        ga_20953: ProductTyped9cc76446c2fc207,
        gs_27303: bool,
        cache: ProductType8ae001a7cc8b5154,
        gs_27272: bool,
        opscope: u32,
        gs_27257: bool,
        ga_20999: ProductTypef170cab34335b70c,
        gs_27300: bool,
        ga_20994: ProductTypeda0231e9dc169f81,
        gs_27302: bool,
        ga_21005: ProductTypeda0231e9dc169f81,
        gs_27273: bool,
        gs_27264: bool,
        ga_21011: ProductType1d757adad216cdef,
        ga_21007: ProductTypeda0231e9dc169f81,
        sizeshadow_505: i128,
        gs_27345: bool,
        gs_27256: bool,
        size: i128,
        vaddress: u64,
        gs_27301: bool,
        gs_27299: bool,
        u_1294: ProductTypece7c66ccb2cab13e,
        ga_21023: ProductTypeda0231e9dc169f81,
        regval: u64,
        cachetype: u32,
        cacheop: u32,
        opscope_in: u32,
    }
    let fn_state = FunctionState {
        regval,
        cachetype,
        cacheop,
        opscope_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var opscope_in:u32
        let s_0_0: u32 = fn_state.opscope_in;
        // D s_0_1: write-var opscope <= s_0_0
        fn_state.opscope = s_0_0;
        // C s_0_2: const #6u : u32
        let s_0_2: u32 = 6;
        // D s_0_3: write-var cache.0 <= s_0_2
        fn_state.cache._0 = s_0_2;
        // D s_0_4: read-var cachetype:u32
        let s_0_4: u32 = fn_state.cachetype;
        // D s_0_5: write-var cache.3 <= s_0_4
        fn_state.cache._3 = s_0_4;
        // D s_0_6: read-var cacheop:u32
        let s_0_6: u32 = fn_state.cacheop;
        // D s_0_7: write-var cache.2 <= s_0_6
        fn_state.cache._2 = s_0_6;
        // D s_0_8: read-var opscope:u32
        let s_0_8: u32 = fn_state.opscope;
        // D s_0_9: write-var cache.8 <= s_0_8
        fn_state.cache._8 = s_0_8;
        // D s_0_10: read-var opscope:u32
        let s_0_10: u32 = fn_state.opscope;
        // C s_0_11: const #0u : u32
        let s_0_11: u32 = 0;
        // D s_0_12: cmp-eq s_0_10 s_0_11
        let s_0_12: bool = ((s_0_10) == (s_0_11));
        // N s_0_13: branch s_0_12 b82 b1
        if s_0_12 {
            return block_82(state, tracer, fn_state);
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
        // N s_1_2: branch s_1_1 b81 b2
        if s_1_1 {
            return block_81(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#27256 <= s_2_0
        fn_state.gs_27256 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#27256:u8
        let s_3_0: bool = fn_state.gs_27256;
        // N s_3_1: branch s_3_0 b75 b4
        if s_3_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_4_1: write-var cache.6 <= s_4_0
        fn_state.cache._6 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16975u : u32
        let s_5_0: u32 = 16975;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #448u : u32
        let s_5_3: u32 = 448;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) == (s_5_5));
        // N s_5_7: branch s_5_6 b74 b6
        if s_5_6 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var cache.5 <= s_6_0
        fn_state.cache._5 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var opscope:u32
        let s_7_0: u32 = fn_state.opscope;
        // C s_7_1: const #5u : u32
        let s_7_1: u32 = 5;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b73 b8
        if s_7_2 {
            return block_73(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#27257 <= s_8_0
        fn_state.gs_27257 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#27257:u8
        let s_9_0: bool = fn_state.gs_27257;
        // N s_9_1: branch s_9_0 b72 b10
        if s_9_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var opscope:u32
        let s_11_0: u32 = fn_state.opscope;
        // C s_11_1: const #4u : u32
        let s_11_1: u32 = 4;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // N s_11_3: branch s_11_2 b71 b12
        if s_11_2 {
            return block_71(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#27258 <= s_12_0
        fn_state.gs_27258 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#27258:u8
        let s_13_0: bool = fn_state.gs_27258;
        // N s_13_1: branch s_13_0 b70 b14
        if s_13_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var regval:u64
        let s_15_0: u64 = fn_state.regval;
        // D s_15_1: write-var vaddress <= s_15_0
        fn_state.vaddress = s_15_0;
        // C s_15_2: const #0s : i
        let s_15_2: i128 = 0;
        // D s_15_3: write-var size <= s_15_2
        fn_state.size = s_15_2;
        // D s_15_4: read-var cacheop:u32
        let s_15_4: u32 = fn_state.cacheop;
        // C s_15_5: const #1u : u32
        let s_15_5: u32 = 1;
        // D s_15_6: cmp-eq s_15_4 s_15_5
        let s_15_6: bool = ((s_15_4) == (s_15_5));
        // N s_15_7: branch s_15_6 b66 b16
        if s_15_6 {
            return block_66(state, tracer, fn_state);
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
        // D s_17_0: read-var size:i
        let s_17_0: i128 = fn_state.size;
        // D s_17_1: write-var sizeshadow#505 <= s_17_0
        fn_state.sizeshadow_505 = s_17_0;
        // D s_17_2: read-var opscope:u32
        let s_17_2: u32 = fn_state.opscope;
        // D s_17_3: call DCInstNeedsTranslation(s_17_2)
        let s_17_3: bool = DCInstNeedsTranslation(state, tracer, s_17_2);
        // N s_17_4: branch s_17_3 b53 b18
        if s_17_3 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var opscope:u32
        let s_18_0: u32 = fn_state.opscope;
        // C s_18_1: const #3u : u32
        let s_18_1: u32 = 3;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b43 b19
        if s_18_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var opscope:u32
        let s_19_0: u32 = fn_state.opscope;
        // C s_19_1: const #6u : u32
        let s_19_1: u32 = 6;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // N s_19_3: branch s_19_2 b42 b20
        if s_19_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var vaddress:u64
        let s_20_0: u64 = fn_state.vaddress;
        // D s_20_1: write-var cache.15 <= s_20_0
        fn_state.cache._15 = s_20_0;
        // C s_20_2: const #0u : u8
        let s_20_2: bool = false;
        // D s_20_3: write-var cache.14 <= s_20_2
        fn_state.cache._14 = s_20_2;
        // C s_20_4: const #() : ()
        let s_20_4: () = ();
        // S s_20_5: call __UNKNOWN_Shareability(s_20_4)
        let s_20_5: u32 = u__UNKNOWN_Shareability(state, tracer, s_20_4);
        // D s_20_6: write-var cache.13 <= s_20_5
        fn_state.cache._13 = s_20_5;
        // C s_20_7: const #() : ()
        let s_20_7: () = ();
        // S s_20_8: call __UNKNOWN_FullAddress(s_20_7)
        let s_20_8: ProductTypeda0231e9dc169f81 = u__UNKNOWN_FullAddress(
            state,
            tracer,
            s_20_7,
        );
        // D s_20_9: write-var cache.9 <= s_20_8
        fn_state.cache._9 = s_20_8;
        // N s_20_10: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var cacheop:u32
        let s_21_0: u32 = fn_state.cacheop;
        // C s_21_1: const #1u : u32
        let s_21_1: u32 = 1;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // N s_21_3: branch s_21_2 b41 b22
        if s_21_2 {
            return block_41(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#27299 <= s_22_0
        fn_state.gs_27299 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#27299:u8
        let s_23_0: bool = fn_state.gs_27299;
        // N s_23_1: branch s_23_0 b40 b24
        if s_23_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#27300 <= s_24_0
        fn_state.gs_27300 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#27300:u8
        let s_25_0: bool = fn_state.gs_27300;
        // N s_25_1: branch s_25_0 b39 b26
        if s_25_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#27301 <= s_26_0
        fn_state.gs_27301 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#27301:u8
        let s_27_0: bool = fn_state.gs_27301;
        // N s_27_1: branch s_27_0 b38 b28
        if s_27_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var cache.14:struct
        let s_29_0: bool = fn_state.cache._14;
        // N s_29_1: branch s_29_0 b37 b30
        if s_29_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#27302 <= s_30_0
        fn_state.gs_27302 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#27302:u8
        let s_31_0: bool = fn_state.gs_27302;
        // N s_31_1: branch s_31_0 b36 b32
        if s_31_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#27303 <= s_32_0
        fn_state.gs_27303 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#27303:u8
        let s_33_0: bool = fn_state.gs_27303;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var cache:struct
        let s_34_0: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_34_1: call CACHE_OP(s_34_0)
        let s_34_1: () = CACHE_OP(state, tracer, s_34_0);
        // N s_34_2: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call HaveSecureState(s_36_0)
        let s_36_1: bool = HaveSecureState(state, tracer, s_36_0);
        // S s_36_2: not s_36_1
        let s_36_2: bool = !s_36_1;
        // D s_36_3: write-var gs#27303 <= s_36_2
        fn_state.gs_27303 = s_36_2;
        // N s_36_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var cache.4:struct
        let s_37_0: u32 = fn_state.cache._4;
        // C s_37_1: const #6u : u32
        let s_37_1: u32 = 6;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: write-var gs#27302 <= s_37_2
        fn_state.gs_27302 = s_37_2;
        // N s_37_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #2u : u32
        let s_38_0: u32 = 2;
        // D s_38_1: write-var cache.2 <= s_38_0
        fn_state.cache._2 = s_38_0;
        // N s_38_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #102552u : u32
        let s_39_0: u32 = 102552;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_HCR_EL2_Type_DC(s_39_1)
        let s_39_2: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_39_1);
        // C s_39_3: const #102552u : u32
        let s_39_3: u32 = 102552;
        // D s_39_4: read-reg s_39_3:struct
        let s_39_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_3 as isize);
            tracer.read_register(s_39_3 as isize, value);
            value
        };
        // D s_39_5: call _get_HCR_EL2_Type_VM(s_39_4)
        let s_39_5: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_39_4);
        // D s_39_6: cast zx s_39_2 -> bv
        let s_39_6: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_7: cast zx s_39_5 -> bv
        let s_39_7: Bits = Bits::new(s_39_5 as u128, 1u16);
        // D s_39_8: cast reint s_39_6 -> u128
        let s_39_8: u128 = (s_39_6.value() as u128);
        // D s_39_9: size-of s_39_6
        let s_39_9: u16 = s_39_6.length();
        // D s_39_10: cast reint s_39_7 -> u128
        let s_39_10: u128 = (s_39_7.value() as u128);
        // D s_39_11: size-of s_39_7
        let s_39_11: u16 = s_39_7.length();
        // D s_39_12: lsl s_39_8 s_39_11
        let s_39_12: u128 = s_39_8 << s_39_11;
        // D s_39_13: or s_39_12 s_39_10
        let s_39_13: u128 = ((s_39_12) | (s_39_10));
        // D s_39_14: add s_39_9 s_39_11
        let s_39_14: u16 = (s_39_9 + s_39_11);
        // D s_39_15: create-bits s_39_13 s_39_14
        let s_39_15: Bits = Bits::new(s_39_13, s_39_14);
        // D s_39_16: cast reint s_39_15 -> u8
        let s_39_16: u8 = (s_39_15.value() as u8);
        // D s_39_17: cast zx s_39_16 -> bv
        let s_39_17: Bits = Bits::new(s_39_16 as u128, 2u16);
        // C s_39_18: const #0u : u8
        let s_39_18: u8 = 0;
        // C s_39_19: cast zx s_39_18 -> bv
        let s_39_19: Bits = Bits::new(s_39_18 as u128, 2u16);
        // D s_39_20: cmp-ne s_39_17 s_39_19
        let s_39_20: bool = ((s_39_17) != (s_39_19));
        // D s_39_21: write-var gs#27301 <= s_39_20
        fn_state.gs_27301 = s_39_20;
        // N s_39_22: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // D s_40_2: write-var gs#27300 <= s_40_1
        fn_state.gs_27300 = s_40_1;
        // N s_40_3: jump b25
        return block_25(state, tracer, fn_state);
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
        // D s_41_7: write-var gs#27299 <= s_41_6
        fn_state.gs_27299 = s_41_6;
        // N s_41_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var cache.14 <= s_42_0
        fn_state.cache._14 = s_42_0;
        // C s_42_2: const #2u : u32
        let s_42_2: u32 = 2;
        // D s_42_3: write-var cache.13 <= s_42_2
        fn_state.cache._13 = s_42_2;
        // C s_42_4: const #0s : i
        let s_42_4: i128 = 0;
        // D s_42_5: read-var regval:u64
        let s_42_5: u64 = fn_state.regval;
        // D s_42_6: cast zx s_42_5 -> bv
        let s_42_6: Bits = Bits::new(s_42_5 as u128, 64u16);
        // C s_42_7: const #1s : i64
        let s_42_7: i64 = 1;
        // C s_42_8: cast zx s_42_7 -> i
        let s_42_8: i128 = (i128::try_from(s_42_7).unwrap());
        // C s_42_9: const #55s : i
        let s_42_9: i128 = 55;
        // C s_42_10: add s_42_9 s_42_8
        let s_42_10: i128 = (s_42_9 + s_42_8);
        // D s_42_11: bit-extract s_42_6 s_42_4 s_42_10
        let s_42_11: Bits = (Bits::new(
            ((s_42_6) >> (s_42_4)).value(),
            u16::try_from(s_42_10).unwrap(),
        ));
        // D s_42_12: cast reint s_42_11 -> u56
        let s_42_12: u64 = (s_42_11.value() as u64);
        // D s_42_13: write-var cache.9.0 <= s_42_12
        fn_state.cache._9._0 = s_42_12;
        // C s_42_14: const #62s : i
        let s_42_14: i128 = 62;
        // D s_42_15: read-var regval:u64
        let s_42_15: u64 = fn_state.regval;
        // D s_42_16: cast zx s_42_15 -> bv
        let s_42_16: Bits = Bits::new(s_42_15 as u128, 64u16);
        // C s_42_17: const #1u : u64
        let s_42_17: u64 = 1;
        // D s_42_18: bit-extract s_42_16 s_42_14 s_42_17
        let s_42_18: Bits = (Bits::new(
            ((s_42_16) >> (s_42_14)).value(),
            u16::try_from(s_42_17).unwrap(),
        ));
        // D s_42_19: cast reint s_42_18 -> u8
        let s_42_19: bool = ((s_42_18.value()) != 0);
        // C s_42_20: const #0s : i
        let s_42_20: i128 = 0;
        // C s_42_21: const #0u : u64
        let s_42_21: u64 = 0;
        // D s_42_22: cast zx s_42_19 -> u64
        let s_42_22: u64 = (s_42_19 as u64);
        // C s_42_23: const #1u : u64
        let s_42_23: u64 = 1;
        // D s_42_24: and s_42_22 s_42_23
        let s_42_24: u64 = ((s_42_22) & (s_42_23));
        // D s_42_25: cmp-eq s_42_24 s_42_23
        let s_42_25: bool = ((s_42_24) == (s_42_23));
        // D s_42_26: lsl s_42_22 s_42_20
        let s_42_26: u64 = s_42_22 << s_42_20;
        // D s_42_27: or s_42_21 s_42_26
        let s_42_27: u64 = ((s_42_21) | (s_42_26));
        // D s_42_28: cmpl s_42_26
        let s_42_28: u64 = !s_42_26;
        // D s_42_29: and s_42_21 s_42_28
        let s_42_29: u64 = ((s_42_21) & (s_42_28));
        // D s_42_30: select s_42_25 s_42_27 s_42_29
        let s_42_30: u64 = if s_42_25 { s_42_27 } else { s_42_29 };
        // D s_42_31: cast trunc s_42_30 -> u8
        let s_42_31: bool = ((s_42_30) != 0);
        // C s_42_32: const #63s : i
        let s_42_32: i128 = 63;
        // D s_42_33: read-var regval:u64
        let s_42_33: u64 = fn_state.regval;
        // D s_42_34: cast zx s_42_33 -> bv
        let s_42_34: Bits = Bits::new(s_42_33 as u128, 64u16);
        // C s_42_35: const #1u : u64
        let s_42_35: u64 = 1;
        // D s_42_36: bit-extract s_42_34 s_42_32 s_42_35
        let s_42_36: Bits = (Bits::new(
            ((s_42_34) >> (s_42_32)).value(),
            u16::try_from(s_42_35).unwrap(),
        ));
        // D s_42_37: cast reint s_42_36 -> u8
        let s_42_37: bool = ((s_42_36.value()) != 0);
        // C s_42_38: const #0s : i
        let s_42_38: i128 = 0;
        // C s_42_39: const #0u : u64
        let s_42_39: u64 = 0;
        // D s_42_40: cast zx s_42_37 -> u64
        let s_42_40: u64 = (s_42_37 as u64);
        // C s_42_41: const #1u : u64
        let s_42_41: u64 = 1;
        // D s_42_42: and s_42_40 s_42_41
        let s_42_42: u64 = ((s_42_40) & (s_42_41));
        // D s_42_43: cmp-eq s_42_42 s_42_41
        let s_42_43: bool = ((s_42_42) == (s_42_41));
        // D s_42_44: lsl s_42_40 s_42_38
        let s_42_44: u64 = s_42_40 << s_42_38;
        // D s_42_45: or s_42_39 s_42_44
        let s_42_45: u64 = ((s_42_39) | (s_42_44));
        // D s_42_46: cmpl s_42_44
        let s_42_46: u64 = !s_42_44;
        // D s_42_47: and s_42_39 s_42_46
        let s_42_47: u64 = ((s_42_39) & (s_42_46));
        // D s_42_48: select s_42_43 s_42_45 s_42_47
        let s_42_48: u64 = if s_42_43 { s_42_45 } else { s_42_47 };
        // D s_42_49: cast trunc s_42_48 -> u8
        let s_42_49: bool = ((s_42_48) != 0);
        // D s_42_50: call DecodePASpace(s_42_31, s_42_49)
        let s_42_50: u32 = DecodePASpace(state, tracer, s_42_31, s_42_49);
        // D s_42_51: write-var cache.9.1 <= s_42_50
        fn_state.cache._9._1 = s_42_50;
        // D s_42_52: read-var cache.9:struct
        let s_42_52: ProductTypeda0231e9dc169f81 = fn_state.cache._9;
        // D s_42_53: write-var ga#21023 <= s_42_52
        fn_state.ga_21023 = s_42_52;
        // D s_42_54: read-var ga#21023.1:struct
        let s_42_54: u32 = fn_state.ga_21023._1;
        // D s_42_55: call CPASAtPAS(s_42_54)
        let s_42_55: u32 = CPASAtPAS(state, tracer, s_42_54);
        // D s_42_56: write-var cache.4 <= s_42_55
        fn_state.cache._4 = s_42_55;
        // N s_42_57: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var cache.14 <= s_43_0
        fn_state.cache._14 = s_43_0;
        // C s_43_2: const #2u : u32
        let s_43_2: u32 = 2;
        // D s_43_3: write-var cache.13 <= s_43_2
        fn_state.cache._13 = s_43_2;
        // C s_43_4: const #0s : i
        let s_43_4: i128 = 0;
        // D s_43_5: read-var regval:u64
        let s_43_5: u64 = fn_state.regval;
        // D s_43_6: cast zx s_43_5 -> bv
        let s_43_6: Bits = Bits::new(s_43_5 as u128, 64u16);
        // C s_43_7: const #1s : i64
        let s_43_7: i64 = 1;
        // C s_43_8: cast zx s_43_7 -> i
        let s_43_8: i128 = (i128::try_from(s_43_7).unwrap());
        // C s_43_9: const #55s : i
        let s_43_9: i128 = 55;
        // C s_43_10: add s_43_9 s_43_8
        let s_43_10: i128 = (s_43_9 + s_43_8);
        // D s_43_11: bit-extract s_43_6 s_43_4 s_43_10
        let s_43_11: Bits = (Bits::new(
            ((s_43_6) >> (s_43_4)).value(),
            u16::try_from(s_43_10).unwrap(),
        ));
        // D s_43_12: cast reint s_43_11 -> u56
        let s_43_12: u64 = (s_43_11.value() as u64);
        // D s_43_13: write-var cache.9.0 <= s_43_12
        fn_state.cache._9._0 = s_43_12;
        // C s_43_14: const #62s : i
        let s_43_14: i128 = 62;
        // D s_43_15: read-var regval:u64
        let s_43_15: u64 = fn_state.regval;
        // D s_43_16: cast zx s_43_15 -> bv
        let s_43_16: Bits = Bits::new(s_43_15 as u128, 64u16);
        // C s_43_17: const #1u : u64
        let s_43_17: u64 = 1;
        // D s_43_18: bit-extract s_43_16 s_43_14 s_43_17
        let s_43_18: Bits = (Bits::new(
            ((s_43_16) >> (s_43_14)).value(),
            u16::try_from(s_43_17).unwrap(),
        ));
        // D s_43_19: cast reint s_43_18 -> u8
        let s_43_19: bool = ((s_43_18.value()) != 0);
        // C s_43_20: const #0s : i
        let s_43_20: i128 = 0;
        // C s_43_21: const #0u : u64
        let s_43_21: u64 = 0;
        // D s_43_22: cast zx s_43_19 -> u64
        let s_43_22: u64 = (s_43_19 as u64);
        // C s_43_23: const #1u : u64
        let s_43_23: u64 = 1;
        // D s_43_24: and s_43_22 s_43_23
        let s_43_24: u64 = ((s_43_22) & (s_43_23));
        // D s_43_25: cmp-eq s_43_24 s_43_23
        let s_43_25: bool = ((s_43_24) == (s_43_23));
        // D s_43_26: lsl s_43_22 s_43_20
        let s_43_26: u64 = s_43_22 << s_43_20;
        // D s_43_27: or s_43_21 s_43_26
        let s_43_27: u64 = ((s_43_21) | (s_43_26));
        // D s_43_28: cmpl s_43_26
        let s_43_28: u64 = !s_43_26;
        // D s_43_29: and s_43_21 s_43_28
        let s_43_29: u64 = ((s_43_21) & (s_43_28));
        // D s_43_30: select s_43_25 s_43_27 s_43_29
        let s_43_30: u64 = if s_43_25 { s_43_27 } else { s_43_29 };
        // D s_43_31: cast trunc s_43_30 -> u8
        let s_43_31: bool = ((s_43_30) != 0);
        // C s_43_32: const #63s : i
        let s_43_32: i128 = 63;
        // D s_43_33: read-var regval:u64
        let s_43_33: u64 = fn_state.regval;
        // D s_43_34: cast zx s_43_33 -> bv
        let s_43_34: Bits = Bits::new(s_43_33 as u128, 64u16);
        // C s_43_35: const #1u : u64
        let s_43_35: u64 = 1;
        // D s_43_36: bit-extract s_43_34 s_43_32 s_43_35
        let s_43_36: Bits = (Bits::new(
            ((s_43_34) >> (s_43_32)).value(),
            u16::try_from(s_43_35).unwrap(),
        ));
        // D s_43_37: cast reint s_43_36 -> u8
        let s_43_37: bool = ((s_43_36.value()) != 0);
        // C s_43_38: const #0s : i
        let s_43_38: i128 = 0;
        // C s_43_39: const #0u : u64
        let s_43_39: u64 = 0;
        // D s_43_40: cast zx s_43_37 -> u64
        let s_43_40: u64 = (s_43_37 as u64);
        // C s_43_41: const #1u : u64
        let s_43_41: u64 = 1;
        // D s_43_42: and s_43_40 s_43_41
        let s_43_42: u64 = ((s_43_40) & (s_43_41));
        // D s_43_43: cmp-eq s_43_42 s_43_41
        let s_43_43: bool = ((s_43_42) == (s_43_41));
        // D s_43_44: lsl s_43_40 s_43_38
        let s_43_44: u64 = s_43_40 << s_43_38;
        // D s_43_45: or s_43_39 s_43_44
        let s_43_45: u64 = ((s_43_39) | (s_43_44));
        // D s_43_46: cmpl s_43_44
        let s_43_46: u64 = !s_43_44;
        // D s_43_47: and s_43_39 s_43_46
        let s_43_47: u64 = ((s_43_39) & (s_43_46));
        // D s_43_48: select s_43_43 s_43_45 s_43_47
        let s_43_48: u64 = if s_43_43 { s_43_45 } else { s_43_47 };
        // D s_43_49: cast trunc s_43_48 -> u8
        let s_43_49: bool = ((s_43_48) != 0);
        // D s_43_50: call DecodePASpace(s_43_31, s_43_49)
        let s_43_50: u32 = DecodePASpace(state, tracer, s_43_31, s_43_49);
        // D s_43_51: write-var cache.9.1 <= s_43_50
        fn_state.cache._9._1 = s_43_50;
        // D s_43_52: read-var cache.9:struct
        let s_43_52: ProductTypeda0231e9dc169f81 = fn_state.cache._9;
        // D s_43_53: write-var ga#21005 <= s_43_52
        fn_state.ga_21005 = s_43_52;
        // D s_43_54: read-var ga#21005.1:struct
        let s_43_54: u32 = fn_state.ga_21005._1;
        // D s_43_55: call CPASAtPAS(s_43_54)
        let s_43_55: u32 = CPASAtPAS(state, tracer, s_43_54);
        // D s_43_56: write-var cache.4 <= s_43_55
        fn_state.cache._4 = s_43_55;
        // D s_43_57: read-var cache.9:struct
        let s_43_57: ProductTypeda0231e9dc169f81 = fn_state.cache._9;
        // D s_43_58: write-var ga#21007 <= s_43_57
        fn_state.ga_21007 = s_43_57;
        // D s_43_59: read-var ga#21007.1:struct
        let s_43_59: u32 = fn_state.ga_21007._1;
        // C s_43_60: const #3u : u32
        let s_43_60: u32 = 3;
        // D s_43_61: cmp-eq s_43_59 s_43_60
        let s_43_61: bool = ((s_43_59) == (s_43_60));
        // N s_43_62: branch s_43_61 b52 b44
        if s_43_61 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #"Apply granule protection check on DC to PoE" : str
        let s_45_0: &'static str = "Apply granule protection check on DC to PoE";
        // S s_45_1: call __IMPDEF_boolean(s_45_0)
        let s_45_1: bool = u__IMPDEF_boolean(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b48 b46
        if s_45_1 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var cache:struct
        let s_48_0: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_48_1: call CreateAccDescDC(s_48_0)
        let s_48_1: ProductType9878976b5bcce9c9 = CreateAccDescDC(state, tracer, s_48_0);
        // D s_48_2: read-var cache.9:struct
        let s_48_2: ProductTypeda0231e9dc169f81 = fn_state.cache._9;
        // D s_48_3: write-var u#1294.3 <= s_48_2
        fn_state.u_1294._3 = s_48_2;
        // D s_48_4: read-var u#1294:struct
        let s_48_4: ProductTypece7c66ccb2cab13e = fn_state.u_1294;
        // D s_48_5: call GranuleProtectionCheck(s_48_4, s_48_1)
        let s_48_5: ProductType396b95aa74979079 = GranuleProtectionCheck(
            state,
            tracer,
            s_48_4,
            s_48_1,
        );
        // D s_48_6: write-var u#1294.0.6 <= s_48_5
        fn_state.u_1294._0._6 = s_48_5;
        // D s_48_7: read-var u#1294.0:struct
        let s_48_7: ProductType1d757adad216cdef = fn_state.u_1294._0;
        // D s_48_8: write-var ga#21011 <= s_48_7
        fn_state.ga_21011 = s_48_7;
        // D s_48_9: read-var ga#21011.6:struct
        let s_48_9: ProductType396b95aa74979079 = fn_state.ga_21011._6;
        // D s_48_10: write-var ga#21012 <= s_48_9
        fn_state.ga_21012 = s_48_9;
        // D s_48_11: read-var ga#21012.0:struct
        let s_48_11: u32 = fn_state.ga_21012._0;
        // C s_48_12: const #0u : u32
        let s_48_12: u32 = 0;
        // D s_48_13: cmp-eq s_48_11 s_48_12
        let s_48_13: bool = ((s_48_11) == (s_48_12));
        // N s_48_14: branch s_48_13 b51 b49
        if s_48_13 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #13u : u32
        let s_51_0: u32 = 13;
        // D s_51_1: write-var u#1294.0.16 <= s_51_0
        fn_state.u_1294._0._16 = s_51_0;
        // D s_51_2: read-var u#1294.3:struct
        let s_51_2: ProductTypeda0231e9dc169f81 = fn_state.u_1294._3;
        // D s_51_3: write-var u#1294.0.12 <= s_51_2
        fn_state.u_1294._0._12 = s_51_2;
        // C s_51_4: const #64s : i64
        let s_51_4: i64 = 64;
        // C s_51_5: cast zx s_51_4 -> i
        let s_51_5: i128 = (i128::try_from(s_51_4).unwrap());
        // S s_51_6: call __UNKNOWN_bits(s_51_5)
        let s_51_6: Bits = u__UNKNOWN_bits(state, tracer, s_51_5);
        // S s_51_7: cast reint s_51_6 -> u64
        let s_51_7: u64 = (s_51_6.value() as u64);
        // D s_51_8: read-var u#1294.0:struct
        let s_51_8: ProductType1d757adad216cdef = fn_state.u_1294._0;
        // D s_51_9: call AArch64_Abort(s_51_7, s_51_8)
        let s_51_9: () = AArch64_Abort(state, tracer, s_51_7, s_51_8);
        // N s_51_10: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EndOfInstruction(s_52_0)
        let s_52_1: () = EndOfInstruction(state, tracer, s_52_0);
        // N s_52_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var vaddress:u64
        let s_53_0: u64 = fn_state.vaddress;
        // D s_53_1: write-var cache.15 <= s_53_0
        fn_state.cache._15 = s_53_0;
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // D s_53_3: read-var cache:struct
        let s_53_3: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_53_4: call CreateAccDescDC(s_53_3)
        let s_53_4: ProductType9878976b5bcce9c9 = CreateAccDescDC(state, tracer, s_53_3);
        // D s_53_5: read-var vaddress:u64
        let s_53_5: u64 = fn_state.vaddress;
        // D s_53_6: read-var sizeshadow#505:i
        let s_53_6: i128 = fn_state.sizeshadow_505;
        // D s_53_7: call AArch64_TranslateAddress(s_53_5, s_53_4, s_53_2, s_53_6)
        let s_53_7: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_53_5,
            s_53_4,
            s_53_2,
            s_53_6,
        );
        // D s_53_8: write-var memaddrdesc <= s_53_7
        fn_state.memaddrdesc = s_53_7;
        // N s_53_9: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var memaddrdesc:struct
        let s_54_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_54_1: call IsFault(s_54_0)
        let s_54_1: bool = IsFault(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b65 b55
        if s_54_1 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var cache.14 <= s_56_0
        fn_state.cache._14 = s_56_0;
        // D s_56_2: read-var memaddrdesc.3:struct
        let s_56_2: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_56_3: write-var cache.9 <= s_56_2
        fn_state.cache._9 = s_56_2;
        // D s_56_4: read-var memaddrdesc.3:struct
        let s_56_4: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_56_5: write-var ga#20994 <= s_56_4
        fn_state.ga_20994 = s_56_4;
        // D s_56_6: read-var ga#20994.1:struct
        let s_56_6: u32 = fn_state.ga_20994._1;
        // D s_56_7: call CPASAtPAS(s_56_6)
        let s_56_7: u32 = CPASAtPAS(state, tracer, s_56_6);
        // D s_56_8: write-var cache.4 <= s_56_7
        fn_state.cache._4 = s_56_7;
        // D s_56_9: read-var opscope:u32
        let s_56_9: u32 = fn_state.opscope;
        // C s_56_10: const #2u : u32
        let s_56_10: u32 = 2;
        // D s_56_11: cmp-eq s_56_9 s_56_10
        let s_56_11: bool = ((s_56_9) == (s_56_10));
        // N s_56_12: branch s_56_11 b64 b57
        if s_56_11 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var opscope:u32
        let s_57_0: u32 = fn_state.opscope;
        // C s_57_1: const #4u : u32
        let s_57_1: u32 = 4;
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // N s_57_3: branch s_57_2 b63 b58
        if s_57_2 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var opscope:u32
        let s_58_0: u32 = fn_state.opscope;
        // C s_58_1: const #5u : u32
        let s_58_1: u32 = 5;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // D s_58_3: write-var gs#27345 <= s_58_2
        fn_state.gs_27345 = s_58_2;
        // N s_58_4: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#27345:u8
        let s_59_0: bool = fn_state.gs_27345;
        // D s_59_1: write-var gs#27346 <= s_59_0
        fn_state.gs_27346 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#27346:u8
        let s_60_0: bool = fn_state.gs_27346;
        // N s_60_1: branch s_60_0 b62 b61
        if s_60_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u32
        let s_61_0: u32 = 0;
        // D s_61_1: write-var cache.13 <= s_61_0
        fn_state.cache._13 = s_61_0;
        // N s_61_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var memaddrdesc.2:struct
        let s_62_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_62_1: write-var ga#20999 <= s_62_0
        fn_state.ga_20999 = s_62_0;
        // D s_62_2: read-var ga#20999.5:struct
        let s_62_2: u32 = fn_state.ga_20999._5;
        // D s_62_3: write-var cache.13 <= s_62_2
        fn_state.cache._13 = s_62_2;
        // N s_62_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#27345 <= s_63_0
        fn_state.gs_27345 = s_63_0;
        // N s_63_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#27346 <= s_64_0
        fn_state.gs_27346 = s_64_0;
        // N s_64_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var memaddrdesc.0:struct
        let s_65_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_65_1: read-var regval:u64
        let s_65_1: u64 = fn_state.regval;
        // D s_65_2: call AArch64_Abort(s_65_1, s_65_0)
        let s_65_2: () = AArch64_Abort(state, tracer, s_65_1, s_65_0);
        // N s_65_3: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #"Data Cache Invalidate Watchpoint Size" : str
        let s_66_0: &'static str = "Data Cache Invalidate Watchpoint Size";
        // S s_66_1: call __IMPDEF_integer(s_66_0)
        let s_66_1: i128 = u__IMPDEF_integer(state, tracer, s_66_0);
        // D s_66_2: write-var size <= s_66_1
        fn_state.size = s_66_1;
        // C s_66_3: const #19056u : u32
        let s_66_3: u32 = 19056;
        // D s_66_4: read-reg s_66_3:struct
        let s_66_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_3 as isize);
            tracer.read_register(s_66_3 as isize, value);
            value
        };
        // D s_66_5: call _get_CTR_EL0_Type_DminLine(s_66_4)
        let s_66_5: u8 = u_get_CTR_EL0_Type_DminLine(state, tracer, s_66_4);
        // D s_66_6: cast zx s_66_5 -> bv
        let s_66_6: Bits = Bits::new(s_66_5 as u128, 4u16);
        // D s_66_7: cast zx s_66_6 -> i
        let s_66_7: i128 = (s_66_6.value() as i128);
        // D s_66_8: cast reint s_66_7 -> i64
        let s_66_8: i64 = (s_66_7 as i64);
        // D s_66_9: cast zx s_66_8 -> i
        let s_66_9: i128 = (i128::try_from(s_66_8).unwrap());
        // D s_66_10: pow2 s_66_9
        let s_66_10: i128 = (s_66_9).pow(2);
        // D s_66_11: cast reint s_66_10 -> i64
        let s_66_11: i64 = (s_66_10 as i64);
        // C s_66_12: const #4s : i
        let s_66_12: i128 = 4;
        // D s_66_13: cast zx s_66_11 -> i
        let s_66_13: i128 = (i128::try_from(s_66_11).unwrap());
        // D s_66_14: mul s_66_12 s_66_13
        let s_66_14: i128 = ((s_66_12) * (s_66_13));
        // D s_66_15: cast reint s_66_14 -> i64
        let s_66_15: i64 = (s_66_14 as i64);
        // D s_66_16: cast zx s_66_15 -> i
        let s_66_16: i128 = (i128::try_from(s_66_15).unwrap());
        // D s_66_17: read-var size:i
        let s_66_17: i128 = fn_state.size;
        // D s_66_18: cmp-ge s_66_17 s_66_16
        let s_66_18: bool = ((s_66_17) >= (s_66_16));
        // N s_66_19: branch s_66_18 b69 b67
        if s_66_18 {
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
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#27288 <= s_67_0
        fn_state.gs_27288 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#27288:u8
        let s_68_0: bool = fn_state.gs_27288;
        // N s_68_1: assert s_68_0
        let s_68_1: () = assert!(s_68_0);
        // C s_68_2: const #32s : i
        let s_68_2: i128 = 32;
        // C s_68_3: const #0s : i
        let s_68_3: i128 = 0;
        // D s_68_4: read-var size:i
        let s_68_4: i128 = fn_state.size;
        // D s_68_5: call integer_subrange(s_68_4, s_68_2, s_68_3)
        let s_68_5: Bits = integer_subrange(state, tracer, s_68_4, s_68_2, s_68_3);
        // D s_68_6: cast reint s_68_5 -> u33
        let s_68_6: u64 = (s_68_5.value() as u64);
        // C s_68_7: const #1s : i
        let s_68_7: i128 = 1;
        // D s_68_8: read-var size:i
        let s_68_8: i128 = fn_state.size;
        // D s_68_9: sub s_68_8 s_68_7
        let s_68_9: i128 = ((s_68_8) - (s_68_7));
        // C s_68_10: const #32s : i
        let s_68_10: i128 = 32;
        // C s_68_11: const #0s : i
        let s_68_11: i128 = 0;
        // D s_68_12: call integer_subrange(s_68_9, s_68_10, s_68_11)
        let s_68_12: Bits = integer_subrange(state, tracer, s_68_9, s_68_10, s_68_11);
        // D s_68_13: cast reint s_68_12 -> u33
        let s_68_13: u64 = (s_68_12.value() as u64);
        // D s_68_14: cast zx s_68_6 -> bv
        let s_68_14: Bits = Bits::new(s_68_6 as u128, 33u16);
        // D s_68_15: cast zx s_68_13 -> bv
        let s_68_15: Bits = Bits::new(s_68_13 as u128, 33u16);
        // D s_68_16: and s_68_14 s_68_15
        let s_68_16: Bits = ((s_68_14) & (s_68_15));
        // D s_68_17: cast reint s_68_16 -> u33
        let s_68_17: u64 = (s_68_16.value() as u64);
        // D s_68_18: cast zx s_68_17 -> bv
        let s_68_18: Bits = Bits::new(s_68_17 as u128, 33u16);
        // D s_68_19: cast zx s_68_18 -> i
        let s_68_19: i128 = (s_68_18.value() as i128);
        // D s_68_20: cast reint s_68_19 -> i64
        let s_68_20: i64 = (s_68_19 as i64);
        // C s_68_21: const #0s : i
        let s_68_21: i128 = 0;
        // D s_68_22: cast zx s_68_20 -> i
        let s_68_22: i128 = (i128::try_from(s_68_20).unwrap());
        // D s_68_23: cmp-eq s_68_22 s_68_21
        let s_68_23: bool = ((s_68_22) == (s_68_21));
        // N s_68_24: assert s_68_23
        let s_68_24: () = assert!(s_68_23);
        // D s_68_25: read-var regval:u64
        let s_68_25: u64 = fn_state.regval;
        // D s_68_26: cast zx s_68_25 -> bv
        let s_68_26: Bits = Bits::new(s_68_25 as u128, 64u16);
        // D s_68_27: read-var size:i
        let s_68_27: i128 = fn_state.size;
        // D s_68_28: call Align_bits(s_68_26, s_68_27)
        let s_68_28: Bits = Align_bits(state, tracer, s_68_26, s_68_27);
        // D s_68_29: cast reint s_68_28 -> u64
        let s_68_29: u64 = (s_68_28.value() as u64);
        // D s_68_30: write-var vaddress <= s_68_29
        fn_state.vaddress = s_68_29;
        // N s_68_31: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #2048s : i
        let s_69_0: i128 = 2048;
        // D s_69_1: read-var size:i
        let s_69_1: i128 = fn_state.size;
        // D s_69_2: cmp-le s_69_1 s_69_0
        let s_69_2: bool = ((s_69_1) <= (s_69_0));
        // D s_69_3: write-var gs#27288 <= s_69_2
        fn_state.gs_27288 = s_69_2;
        // N s_69_4: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #2u : u32
        let s_70_0: u32 = 2;
        // D s_70_1: write-var opscope <= s_70_0
        fn_state.opscope = s_70_0;
        // N s_70_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #"Memory system does not supports PoP" : str
        let s_71_0: &'static str = "Memory system does not supports PoP";
        // S s_71_1: call __IMPDEF_boolean(s_71_0)
        let s_71_1: bool = u__IMPDEF_boolean(state, tracer, s_71_0);
        // D s_71_2: write-var gs#27258 <= s_71_1
        fn_state.gs_27258 = s_71_1;
        // N s_71_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #4u : u32
        let s_72_0: u32 = 4;
        // D s_72_1: write-var opscope <= s_72_0
        fn_state.opscope = s_72_0;
        // N s_72_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #"Memory system does not supports PoDP" : str
        let s_73_0: &'static str = "Memory system does not supports PoDP";
        // S s_73_1: call __IMPDEF_boolean(s_73_0)
        let s_73_1: bool = u__IMPDEF_boolean(state, tracer, s_73_0);
        // D s_73_2: write-var gs#27257 <= s_73_1
        fn_state.gs_27257 = s_73_1;
        // N s_73_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var cache.5 <= s_74_0
        fn_state.cache._5 = s_74_0;
        // C s_74_2: const #() : ()
        let s_74_2: () = ();
        // S s_74_3: call ASID_read(s_74_2)
        let s_74_3: u16 = ASID_read(state, tracer, s_74_2);
        // D s_74_4: write-var cache.1 <= s_74_3
        fn_state.cache._1 = s_74_3;
        // N s_74_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #16975u : u32
        let s_75_0: u32 = 16975;
        // D s_75_1: read-reg s_75_0:u8
        let s_75_1: u8 = {
            let value = state.read_register::<u8>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: cast zx s_75_1 -> bv
        let s_75_2: Bits = Bits::new(s_75_1 as u128, 2u16);
        // C s_75_3: const #448u : u32
        let s_75_3: u32 = 448;
        // D s_75_4: read-reg s_75_3:u8
        let s_75_4: u8 = {
            let value = state.read_register::<u8>(s_75_3 as isize);
            tracer.read_register(s_75_3 as isize, value);
            value
        };
        // D s_75_5: cast zx s_75_4 -> bv
        let s_75_5: Bits = Bits::new(s_75_4 as u128, 2u16);
        // D s_75_6: cmp-eq s_75_2 s_75_5
        let s_75_6: bool = ((s_75_2) == (s_75_5));
        // N s_75_7: branch s_75_6 b80 b76
        if s_75_6 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
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
        // D s_76_7: write-var gs#27264 <= s_76_6
        fn_state.gs_27264 = s_76_6;
        // N s_76_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#27264:u8
        let s_77_0: bool = fn_state.gs_27264;
        // N s_77_1: branch s_77_0 b79 b78
        if s_77_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var cache.6 <= s_78_0
        fn_state.cache._6 = s_78_0;
        // N s_78_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var cache.6 <= s_79_0
        fn_state.cache._6 = s_79_0;
        // C s_79_2: const #() : ()
        let s_79_2: () = ();
        // S s_79_3: call VMID_read(s_79_2)
        let s_79_3: u16 = VMID_read(state, tracer, s_79_2);
        // D s_79_4: write-var cache.16 <= s_79_3
        fn_state.cache._16 = s_79_3;
        // N s_79_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#27264 <= s_80_0
        fn_state.gs_27264 = s_80_0;
        // N s_80_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #() : ()
        let s_81_0: () = ();
        // S s_81_1: call IsInHost(s_81_0)
        let s_81_1: bool = IsInHost(state, tracer, s_81_0);
        // S s_81_2: not s_81_1
        let s_81_2: bool = !s_81_1;
        // D s_81_3: write-var gs#27256 <= s_81_2
        fn_state.gs_27256 = s_81_2;
        // N s_81_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #16975u : u32
        let s_82_0: u32 = 16975;
        // D s_82_1: read-reg s_82_0:u8
        let s_82_1: u8 = {
            let value = state.read_register::<u8>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call SecurityStateAtEL(s_82_1)
        let s_82_2: u32 = SecurityStateAtEL(state, tracer, s_82_1);
        // D s_82_3: call CPASAtSecurityState(s_82_2)
        let s_82_3: u32 = CPASAtSecurityState(state, tracer, s_82_2);
        // D s_82_4: write-var cache.4 <= s_82_3
        fn_state.cache._4 = s_82_3;
        // C s_82_5: const #0u : u32
        let s_82_5: u32 = 0;
        // D s_82_6: write-var cache.13 <= s_82_5
        fn_state.cache._13 = s_82_5;
        // D s_82_7: read-var regval:u64
        let s_82_7: u64 = fn_state.regval;
        // D s_82_8: read-var cachetype:u32
        let s_82_8: u32 = fn_state.cachetype;
        // D s_82_9: call DecodeSW(s_82_7, s_82_8)
        let s_82_9: ProductTyped9cc76446c2fc207 = DecodeSW(
            state,
            tracer,
            s_82_7,
            s_82_8,
        );
        // D s_82_10: write-var ga#20953 <= s_82_9
        fn_state.ga_20953 = s_82_9;
        // D s_82_11: read-var ga#20953.0:struct
        let s_82_11: i128 = fn_state.ga_20953._0;
        // D s_82_12: read-var ga#20953.1:struct
        let s_82_12: i128 = fn_state.ga_20953._1;
        // D s_82_13: read-var ga#20953.2:struct
        let s_82_13: i128 = fn_state.ga_20953._2;
        // D s_82_14: write-var cache.12 <= s_82_11
        fn_state.cache._12 = s_82_11;
        // D s_82_15: write-var cache.17 <= s_82_12
        fn_state.cache._17 = s_82_12;
        // D s_82_16: write-var cache.7 <= s_82_13
        fn_state.cache._7 = s_82_13;
        // D s_82_17: read-var cacheop:u32
        let s_82_17: u32 = fn_state.cacheop;
        // C s_82_18: const #1u : u32
        let s_82_18: u32 = 1;
        // D s_82_19: cmp-eq s_82_17 s_82_18
        let s_82_19: bool = ((s_82_17) == (s_82_18));
        // N s_82_20: branch s_82_19 b97 b83
        if s_82_19 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#27271 <= s_83_0
        fn_state.gs_27271 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#27271:u8
        let s_84_0: bool = fn_state.gs_27271;
        // N s_84_1: branch s_84_0 b96 b85
        if s_84_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#27272 <= s_85_0
        fn_state.gs_27272 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#27272:u8
        let s_86_0: bool = fn_state.gs_27272;
        // N s_86_1: branch s_86_0 b92 b87
        if s_86_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#27274 <= s_87_0
        fn_state.gs_27274 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#27274:u8
        let s_88_0: bool = fn_state.gs_27274;
        // N s_88_1: branch s_88_0 b91 b89
        if s_88_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_89_0: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var cache:struct
        let s_90_0: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_90_1: call CACHE_OP(s_90_0)
        let s_90_1: () = CACHE_OP(state, tracer, s_90_0);
        // N s_90_2: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #2u : u32
        let s_91_0: u32 = 2;
        // D s_91_1: write-var cache.2 <= s_91_0
        fn_state.cache._2 = s_91_0;
        // N s_91_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #102552u : u32
        let s_92_0: u32 = 102552;
        // D s_92_1: read-reg s_92_0:struct
        let s_92_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call _get_HCR_EL2_Type_SWIO(s_92_1)
        let s_92_2: bool = u_get_HCR_EL2_Type_SWIO(state, tracer, s_92_1);
        // D s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // C s_92_4: const #1u : u8
        let s_92_4: bool = true;
        // C s_92_5: cast zx s_92_4 -> bv
        let s_92_5: Bits = Bits::new(s_92_4 as u128, 1u16);
        // D s_92_6: cmp-eq s_92_3 s_92_5
        let s_92_6: bool = ((s_92_3) == (s_92_5));
        // N s_92_7: branch s_92_6 b95 b93
        if s_92_6 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #102552u : u32
        let s_93_0: u32 = 102552;
        // D s_93_1: read-reg s_93_0:struct
        let s_93_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call _get_HCR_EL2_Type_DC(s_93_1)
        let s_93_2: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_93_1);
        // C s_93_3: const #102552u : u32
        let s_93_3: u32 = 102552;
        // D s_93_4: read-reg s_93_3:struct
        let s_93_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_93_3 as isize);
            tracer.read_register(s_93_3 as isize, value);
            value
        };
        // D s_93_5: call _get_HCR_EL2_Type_VM(s_93_4)
        let s_93_5: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_93_4);
        // D s_93_6: cast zx s_93_2 -> bv
        let s_93_6: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_7: cast zx s_93_5 -> bv
        let s_93_7: Bits = Bits::new(s_93_5 as u128, 1u16);
        // D s_93_8: cast reint s_93_6 -> u128
        let s_93_8: u128 = (s_93_6.value() as u128);
        // D s_93_9: size-of s_93_6
        let s_93_9: u16 = s_93_6.length();
        // D s_93_10: cast reint s_93_7 -> u128
        let s_93_10: u128 = (s_93_7.value() as u128);
        // D s_93_11: size-of s_93_7
        let s_93_11: u16 = s_93_7.length();
        // D s_93_12: lsl s_93_8 s_93_11
        let s_93_12: u128 = s_93_8 << s_93_11;
        // D s_93_13: or s_93_12 s_93_10
        let s_93_13: u128 = ((s_93_12) | (s_93_10));
        // D s_93_14: add s_93_9 s_93_11
        let s_93_14: u16 = (s_93_9 + s_93_11);
        // D s_93_15: create-bits s_93_13 s_93_14
        let s_93_15: Bits = Bits::new(s_93_13, s_93_14);
        // D s_93_16: cast reint s_93_15 -> u8
        let s_93_16: u8 = (s_93_15.value() as u8);
        // D s_93_17: cast zx s_93_16 -> bv
        let s_93_17: Bits = Bits::new(s_93_16 as u128, 2u16);
        // C s_93_18: const #0u : u8
        let s_93_18: u8 = 0;
        // C s_93_19: cast zx s_93_18 -> bv
        let s_93_19: Bits = Bits::new(s_93_18 as u128, 2u16);
        // D s_93_20: cmp-ne s_93_17 s_93_19
        let s_93_20: bool = ((s_93_17) != (s_93_19));
        // D s_93_21: write-var gs#27273 <= s_93_20
        fn_state.gs_27273 = s_93_20;
        // N s_93_22: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#27273:u8
        let s_94_0: bool = fn_state.gs_27273;
        // D s_94_1: write-var gs#27274 <= s_94_0
        fn_state.gs_27274 = s_94_0;
        // N s_94_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #1u : u8
        let s_95_0: bool = true;
        // D s_95_1: write-var gs#27273 <= s_95_0
        fn_state.gs_27273 = s_95_0;
        // N s_95_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call EL2Enabled(s_96_0)
        let s_96_1: bool = EL2Enabled(state, tracer, s_96_0);
        // D s_96_2: write-var gs#27272 <= s_96_1
        fn_state.gs_27272 = s_96_1;
        // N s_96_3: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #16975u : u32
        let s_97_0: u32 = 16975;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: cast zx s_97_1 -> bv
        let s_97_2: Bits = Bits::new(s_97_1 as u128, 2u16);
        // C s_97_3: const #440u : u32
        let s_97_3: u32 = 440;
        // D s_97_4: read-reg s_97_3:u8
        let s_97_4: u8 = {
            let value = state.read_register::<u8>(s_97_3 as isize);
            tracer.read_register(s_97_3 as isize, value);
            value
        };
        // D s_97_5: cast zx s_97_4 -> bv
        let s_97_5: Bits = Bits::new(s_97_4 as u128, 2u16);
        // D s_97_6: cmp-eq s_97_2 s_97_5
        let s_97_6: bool = ((s_97_2) == (s_97_5));
        // D s_97_7: write-var gs#27271 <= s_97_6
        fn_state.gs_27271 = s_97_6;
        // N s_97_8: jump b84
        return block_84(state, tracer, fn_state);
    }
}
