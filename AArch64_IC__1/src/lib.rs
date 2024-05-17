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
use ICInstNeedsTranslation::*;
use VMID_read::*;
use u__UNKNOWN_FullAddress::*;
use AArch64_TranslateAddress::*;
use ASID_read::*;
use CACHE_OP::*;
use CPASAtPAS::*;
use SecurityStateAtEL::*;
use CPASAtSecurityState::*;
use AArch64_Abort::*;
use IsFault::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_FB::*;
use CreateAccDescIC::*;
use common::*;
pub fn AArch64_IC__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regval: u64,
    opscope: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_27363: bool,
        ga_21075: ProductTypeda0231e9dc169f81,
        gs_27382: bool,
        cache: ProductType8ae001a7cc8b5154,
        gs_27385: bool,
        gs_27383: bool,
        gs_27367: bool,
        vaddress: u64,
        gs_27361: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        gs_27384: bool,
        regval: u64,
        opscope: u32,
    }
    let fn_state = FunctionState {
        regval,
        opscope,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #5u : u32
        let s_0_0: u32 = 5;
        // D s_0_1: write-var cache.0 <= s_0_0
        fn_state.cache._0 = s_0_0;
        // C s_0_2: const #3u : u32
        let s_0_2: u32 = 3;
        // D s_0_3: write-var cache.3 <= s_0_2
        fn_state.cache._3 = s_0_2;
        // C s_0_4: const #1u : u32
        let s_0_4: u32 = 1;
        // D s_0_5: write-var cache.2 <= s_0_4
        fn_state.cache._2 = s_0_4;
        // D s_0_6: read-var opscope:u32
        let s_0_6: u32 = fn_state.opscope;
        // D s_0_7: write-var cache.8 <= s_0_6
        fn_state.cache._8 = s_0_6;
        // D s_0_8: read-var opscope:u32
        let s_0_8: u32 = fn_state.opscope;
        // C s_0_9: const #7u : u32
        let s_0_9: u32 = 7;
        // D s_0_10: cmp-eq s_0_8 s_0_9
        let s_0_10: bool = ((s_0_8) == (s_0_9));
        // N s_0_11: branch s_0_10 b40 b1
        if s_0_10 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var opscope:u32
        let s_1_0: u32 = fn_state.opscope;
        // C s_1_1: const #8u : u32
        let s_1_1: u32 = 8;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: write-var gs#27361 <= s_1_2
        fn_state.gs_27361 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#27361:u8
        let s_2_0: bool = fn_state.gs_27361;
        // N s_2_1: branch s_2_0 b24 b3
        if s_2_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var opscope:u32
        let s_3_0: u32 = fn_state.opscope;
        // C s_3_1: const #1u : u32
        let s_3_1: u32 = 1;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: assert s_3_2
        let s_3_3: () = assert!(s_3_2);
        // C s_3_4: const #() : ()
        let s_3_4: () = ();
        // S s_3_5: call EL2Enabled(s_3_4)
        let s_3_5: bool = EL2Enabled(state, tracer, s_3_4);
        // N s_3_6: branch s_3_5 b23 b4
        if s_3_5 {
            return block_23(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#27363 <= s_4_0
        fn_state.gs_27363 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#27363:u8
        let s_5_0: bool = fn_state.gs_27363;
        // N s_5_1: branch s_5_0 b17 b6
        if s_5_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_6_1: write-var cache.6 <= s_6_0
        fn_state.cache._6 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16975u : u32
        let s_7_0: u32 = 16975;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 2u16);
        // C s_7_3: const #448u : u32
        let s_7_3: u32 = 448;
        // D s_7_4: read-reg s_7_3:u8
        let s_7_4: u8 = {
            let value = state.read_register::<u8>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 2u16);
        // D s_7_6: cmp-eq s_7_2 s_7_5
        let s_7_6: bool = ((s_7_2) == (s_7_5));
        // N s_7_7: branch s_7_6 b16 b8
        if s_7_6 {
            return block_16(state, tracer, fn_state);
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
        // D s_8_1: write-var cache.5 <= s_8_0
        fn_state.cache._5 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var regval:u64
        let s_9_0: u64 = fn_state.regval;
        // D s_9_1: write-var vaddress <= s_9_0
        fn_state.vaddress = s_9_0;
        // D s_9_2: read-var opscope:u32
        let s_9_2: u32 = fn_state.opscope;
        // D s_9_3: call ICInstNeedsTranslation(s_9_2)
        let s_9_3: bool = ICInstNeedsTranslation(state, tracer, s_9_2);
        // D s_9_4: read-var regval:u64
        let s_9_4: u64 = fn_state.regval;
        // D s_9_5: write-var cache.15 <= s_9_4
        fn_state.cache._15 = s_9_4;
        // C s_9_6: const #0u : u32
        let s_9_6: u32 = 0;
        // D s_9_7: write-var cache.13 <= s_9_6
        fn_state.cache._13 = s_9_6;
        // D s_9_8: write-var cache.14 <= s_9_3
        fn_state.cache._14 = s_9_3;
        // D s_9_9: not s_9_3
        let s_9_9: bool = !s_9_3;
        // N s_9_10: branch s_9_9 b15 b10
        if s_9_9 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var cache:struct
        let s_10_0: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_10_1: call CreateAccDescIC(s_10_0)
        let s_10_1: ProductType9878976b5bcce9c9 = CreateAccDescIC(state, tracer, s_10_0);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: const #0s : i64
        let s_10_3: i64 = 0;
        // C s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: read-var vaddress:u64
        let s_10_5: u64 = fn_state.vaddress;
        // D s_10_6: call AArch64_TranslateAddress(s_10_5, s_10_1, s_10_2, s_10_4)
        let s_10_6: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_10_5,
            s_10_1,
            s_10_2,
            s_10_4,
        );
        // D s_10_7: write-var memaddrdesc <= s_10_6
        fn_state.memaddrdesc = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var memaddrdesc:struct
        let s_11_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_11_1: call IsFault(s_11_0)
        let s_11_1: bool = IsFault(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b14 b12
        if s_11_1 {
            return block_14(state, tracer, fn_state);
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
        // D s_13_0: read-var memaddrdesc.3:struct
        let s_13_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_13_1: write-var ga#21075 <= s_13_0
        fn_state.ga_21075 = s_13_0;
        // D s_13_2: read-var ga#21075.1:struct
        let s_13_2: u32 = fn_state.ga_21075._1;
        // D s_13_3: call CPASAtPAS(s_13_2)
        let s_13_3: u32 = CPASAtPAS(state, tracer, s_13_2);
        // D s_13_4: write-var cache.4 <= s_13_3
        fn_state.cache._4 = s_13_3;
        // D s_13_5: read-var memaddrdesc.3:struct
        let s_13_5: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_13_6: write-var cache.9 <= s_13_5
        fn_state.cache._9 = s_13_5;
        // D s_13_7: read-var cache:struct
        let s_13_7: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_13_8: call CACHE_OP(s_13_7)
        let s_13_8: () = CACHE_OP(state, tracer, s_13_7);
        // N s_13_9: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var memaddrdesc.0:struct
        let s_14_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_14_1: read-var regval:u64
        let s_14_1: u64 = fn_state.regval;
        // D s_14_2: call AArch64_Abort(s_14_1, s_14_0)
        let s_14_2: () = AArch64_Abort(state, tracer, s_14_1, s_14_0);
        // N s_14_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call __UNKNOWN_FullAddress(s_15_0)
        let s_15_1: ProductTypeda0231e9dc169f81 = u__UNKNOWN_FullAddress(
            state,
            tracer,
            s_15_0,
        );
        // D s_15_2: write-var cache.9 <= s_15_1
        fn_state.cache._9 = s_15_1;
        // D s_15_3: read-var cache:struct
        let s_15_3: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_15_4: call CACHE_OP(s_15_3)
        let s_15_4: () = CACHE_OP(state, tracer, s_15_3);
        // N s_15_5: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var cache.5 <= s_16_0
        fn_state.cache._5 = s_16_0;
        // C s_16_2: const #() : ()
        let s_16_2: () = ();
        // S s_16_3: call ASID_read(s_16_2)
        let s_16_3: u16 = ASID_read(state, tracer, s_16_2);
        // D s_16_4: write-var cache.1 <= s_16_3
        fn_state.cache._1 = s_16_3;
        // N s_16_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #16975u : u32
        let s_17_0: u32 = 16975;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 2u16);
        // C s_17_3: const #448u : u32
        let s_17_3: u32 = 448;
        // D s_17_4: read-reg s_17_3:u8
        let s_17_4: u8 = {
            let value = state.read_register::<u8>(s_17_3 as isize);
            tracer.read_register(s_17_3 as isize, value);
            value
        };
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 2u16);
        // D s_17_6: cmp-eq s_17_2 s_17_5
        let s_17_6: bool = ((s_17_2) == (s_17_5));
        // N s_17_7: branch s_17_6 b22 b18
        if s_17_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #16975u : u32
        let s_18_0: u32 = 16975;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 2u16);
        // C s_18_3: const #440u : u32
        let s_18_3: u32 = 440;
        // D s_18_4: read-reg s_18_3:u8
        let s_18_4: u8 = {
            let value = state.read_register::<u8>(s_18_3 as isize);
            tracer.read_register(s_18_3 as isize, value);
            value
        };
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 2u16);
        // D s_18_6: cmp-eq s_18_2 s_18_5
        let s_18_6: bool = ((s_18_2) == (s_18_5));
        // D s_18_7: write-var gs#27367 <= s_18_6
        fn_state.gs_27367 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#27367:u8
        let s_19_0: bool = fn_state.gs_27367;
        // N s_19_1: branch s_19_0 b21 b20
        if s_19_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_20_1: write-var cache.6 <= s_20_0
        fn_state.cache._6 = s_20_0;
        // N s_20_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var cache.6 <= s_21_0
        fn_state.cache._6 = s_21_0;
        // C s_21_2: const #() : ()
        let s_21_2: () = ();
        // S s_21_3: call VMID_read(s_21_2)
        let s_21_3: u16 = VMID_read(state, tracer, s_21_2);
        // D s_21_4: write-var cache.16 <= s_21_3
        fn_state.cache._16 = s_21_3;
        // N s_21_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#27367 <= s_22_0
        fn_state.gs_27367 = s_22_0;
        // N s_22_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call IsInHost(s_23_0)
        let s_23_1: bool = IsInHost(state, tracer, s_23_0);
        // S s_23_2: not s_23_1
        let s_23_2: bool = !s_23_1;
        // D s_23_3: write-var gs#27363 <= s_23_2
        fn_state.gs_27363 = s_23_2;
        // N s_23_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #16975u : u32
        let s_24_0: u32 = 16975;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call SecurityStateAtEL(s_24_1)
        let s_24_2: u32 = SecurityStateAtEL(state, tracer, s_24_1);
        // D s_24_3: call CPASAtSecurityState(s_24_2)
        let s_24_3: u32 = CPASAtSecurityState(state, tracer, s_24_2);
        // D s_24_4: write-var cache.4 <= s_24_3
        fn_state.cache._4 = s_24_3;
        // D s_24_5: read-var opscope:u32
        let s_24_5: u32 = fn_state.opscope;
        // C s_24_6: const #8u : u32
        let s_24_6: u32 = 8;
        // D s_24_7: cmp-eq s_24_5 s_24_6
        let s_24_7: bool = ((s_24_5) == (s_24_6));
        // N s_24_8: branch s_24_7 b39 b25
        if s_24_7 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var opscope:u32
        let s_25_0: u32 = fn_state.opscope;
        // C s_25_1: const #7u : u32
        let s_25_1: u32 = 7;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // N s_25_3: branch s_25_2 b38 b26
        if s_25_2 {
            return block_38(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#27382 <= s_26_0
        fn_state.gs_27382 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#27382:u8
        let s_27_0: bool = fn_state.gs_27382;
        // N s_27_1: branch s_27_0 b37 b28
        if s_27_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#27383 <= s_28_0
        fn_state.gs_27383 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#27383:u8
        let s_29_0: bool = fn_state.gs_27383;
        // N s_29_1: branch s_29_0 b36 b30
        if s_29_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#27384 <= s_30_0
        fn_state.gs_27384 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#27384:u8
        let s_31_0: bool = fn_state.gs_27384;
        // D s_31_1: write-var gs#27385 <= s_31_0
        fn_state.gs_27385 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#27385:u8
        let s_32_0: bool = fn_state.gs_27385;
        // N s_32_1: branch s_32_0 b35 b33
        if s_32_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u32
        let s_33_0: u32 = 0;
        // D s_33_1: write-var cache.13 <= s_33_0
        fn_state.cache._13 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var regval:u64
        let s_34_0: u64 = fn_state.regval;
        // D s_34_1: write-var cache.10 <= s_34_0
        fn_state.cache._10 = s_34_0;
        // D s_34_2: read-var cache:struct
        let s_34_2: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_34_3: call CACHE_OP(s_34_2)
        let s_34_3: () = CACHE_OP(state, tracer, s_34_2);
        // N s_34_4: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u32
        let s_35_0: u32 = 1;
        // D s_35_1: write-var cache.13 <= s_35_0
        fn_state.cache._13 = s_35_0;
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #102552u : u32
        let s_36_0: u32 = 102552;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_HCR_EL2_Type_FB(s_36_1)
        let s_36_2: bool = u_get_HCR_EL2_Type_FB(state, tracer, s_36_1);
        // D s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // D s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#27384 <= s_36_6
        fn_state.gs_27384 = s_36_6;
        // N s_36_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call EL2Enabled(s_37_0)
        let s_37_1: bool = EL2Enabled(state, tracer, s_37_0);
        // D s_37_2: write-var gs#27383 <= s_37_1
        fn_state.gs_27383 = s_37_1;
        // N s_37_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #16975u : u32
        let s_38_0: u32 = 16975;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: cast zx s_38_1 -> bv
        let s_38_2: Bits = Bits::new(s_38_1 as u128, 2u16);
        // C s_38_3: const #440u : u32
        let s_38_3: u32 = 440;
        // D s_38_4: read-reg s_38_3:u8
        let s_38_4: u8 = {
            let value = state.read_register::<u8>(s_38_3 as isize);
            tracer.read_register(s_38_3 as isize, value);
            value
        };
        // D s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 2u16);
        // D s_38_6: cmp-eq s_38_2 s_38_5
        let s_38_6: bool = ((s_38_2) == (s_38_5));
        // D s_38_7: write-var gs#27382 <= s_38_6
        fn_state.gs_27382 = s_38_6;
        // N s_38_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#27385 <= s_39_0
        fn_state.gs_27385 = s_39_0;
        // N s_39_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#27361 <= s_40_0
        fn_state.gs_27361 = s_40_0;
        // N s_40_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
