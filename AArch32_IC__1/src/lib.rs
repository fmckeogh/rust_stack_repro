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
use u_get_HCR_Type_FB::*;
use ICInstNeedsTranslation::*;
use AArch32_Abort::*;
use ASID_read::*;
use CACHE_OP::*;
use HCR_read::*;
use SecurityStateAtEL::*;
use IsFault::*;
use VMID_read::*;
use EL2Enabled::*;
use AArch32_TranslateAddress::*;
use u__UNKNOWN_FullAddress::*;
use CreateAccDescIC::*;
use common::*;
pub fn AArch32_IC__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regval: u32,
    opscope: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_30342: bool,
        gs_30343: bool,
        cache: ProductType8ae001a7cc8b5154,
        gs_30326: bool,
        gs_30321: bool,
        gs_30341: bool,
        gs_30344: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        regval: u32,
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
        // C s_0_13: const #7u : u32
        let s_0_13: u32 = 7;
        // D s_0_14: cmp-eq s_0_12 s_0_13
        let s_0_14: bool = ((s_0_12) == (s_0_13));
        // N s_0_15: branch s_0_14 b37 b1
        if s_0_14 {
            return block_37(state, tracer, fn_state);
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
        // D s_1_3: write-var gs#30321 <= s_1_2
        fn_state.gs_30321 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#30321:u8
        let s_2_0: bool = fn_state.gs_30321;
        // N s_2_1: branch s_2_0 b21 b3
        if s_2_0 {
            return block_21(state, tracer, fn_state);
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
        // N s_3_6: branch s_3_5 b15 b4
        if s_3_5 {
            return block_15(state, tracer, fn_state);
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
        // N s_5_7: branch s_5_6 b14 b6
        if s_5_6 {
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
        // D s_7_1: call ICInstNeedsTranslation(s_7_0)
        let s_7_1: bool = ICInstNeedsTranslation(state, tracer, s_7_0);
        // C s_7_2: const #0u : u32
        let s_7_2: u32 = 0;
        // D s_7_3: write-var cache.13 <= s_7_2
        fn_state.cache._13 = s_7_2;
        // C s_7_4: const #64s : i
        let s_7_4: i128 = 64;
        // D s_7_5: read-var regval:u32
        let s_7_5: u32 = fn_state.regval;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 32u16);
        // D s_7_7: bits-cast zx s_7_6 -> bv length s_7_4
        let s_7_7: Bits = s_7_6.zero_extend(s_7_4);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // D s_7_9: write-var cache.15 <= s_7_8
        fn_state.cache._15 = s_7_8;
        // D s_7_10: write-var cache.14 <= s_7_1
        fn_state.cache._14 = s_7_1;
        // D s_7_11: not s_7_1
        let s_7_11: bool = !s_7_1;
        // N s_7_12: branch s_7_11 b13 b8
        if s_7_11 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i64
        let s_8_0: i64 = 0;
        // C s_8_1: const #1u : u8
        let s_8_1: bool = true;
        // D s_8_2: read-var cache:struct
        let s_8_2: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_8_3: call CreateAccDescIC(s_8_2)
        let s_8_3: ProductType9878976b5bcce9c9 = CreateAccDescIC(state, tracer, s_8_2);
        // C s_8_4: cast zx s_8_0 -> i
        let s_8_4: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_5: read-var regval:u32
        let s_8_5: u32 = fn_state.regval;
        // D s_8_6: call AArch32_TranslateAddress(s_8_5, s_8_3, s_8_1, s_8_4)
        let s_8_6: ProductTypece7c66ccb2cab13e = AArch32_TranslateAddress(
            state,
            tracer,
            s_8_5,
            s_8_3,
            s_8_1,
            s_8_4,
        );
        // D s_8_7: write-var memaddrdesc <= s_8_6
        fn_state.memaddrdesc = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var memaddrdesc:struct
        let s_9_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_9_1: call IsFault(s_9_0)
        let s_9_1: bool = IsFault(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b12 b10
        if s_9_1 {
            return block_12(state, tracer, fn_state);
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
        // D s_11_0: read-var memaddrdesc.3:struct
        let s_11_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_11_1: write-var cache.9 <= s_11_0
        fn_state.cache._9 = s_11_0;
        // D s_11_2: read-var cache:struct
        let s_11_2: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_11_3: call CACHE_OP(s_11_2)
        let s_11_3: () = CACHE_OP(state, tracer, s_11_2);
        // N s_11_4: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var memaddrdesc.0:struct
        let s_12_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_12_1: read-var regval:u32
        let s_12_1: u32 = fn_state.regval;
        // D s_12_2: call AArch32_Abort(s_12_1, s_12_0)
        let s_12_2: () = AArch32_Abort(state, tracer, s_12_1, s_12_0);
        // N s_12_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call __UNKNOWN_FullAddress(s_13_0)
        let s_13_1: ProductTypeda0231e9dc169f81 = u__UNKNOWN_FullAddress(
            state,
            tracer,
            s_13_0,
        );
        // D s_13_2: write-var cache.9 <= s_13_1
        fn_state.cache._9 = s_13_1;
        // D s_13_3: read-var cache:struct
        let s_13_3: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_13_4: call CACHE_OP(s_13_3)
        let s_13_4: () = CACHE_OP(state, tracer, s_13_3);
        // N s_13_5: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var cache.5 <= s_14_0
        fn_state.cache._5 = s_14_0;
        // C s_14_2: const #() : ()
        let s_14_2: () = ();
        // S s_14_3: call ASID_read(s_14_2)
        let s_14_3: u16 = ASID_read(state, tracer, s_14_2);
        // D s_14_4: write-var cache.1 <= s_14_3
        fn_state.cache._1 = s_14_3;
        // N s_14_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16975u : u32
        let s_15_0: u32 = 16975;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 2u16);
        // C s_15_3: const #448u : u32
        let s_15_3: u32 = 448;
        // D s_15_4: read-reg s_15_3:u8
        let s_15_4: u8 = {
            let value = state.read_register::<u8>(s_15_3 as isize);
            tracer.read_register(s_15_3 as isize, value);
            value
        };
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // D s_15_6: cmp-eq s_15_2 s_15_5
        let s_15_6: bool = ((s_15_2) == (s_15_5));
        // N s_15_7: branch s_15_6 b20 b16
        if s_15_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16975u : u32
        let s_16_0: u32 = 16975;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 2u16);
        // C s_16_3: const #440u : u32
        let s_16_3: u32 = 440;
        // D s_16_4: read-reg s_16_3:u8
        let s_16_4: u8 = {
            let value = state.read_register::<u8>(s_16_3 as isize);
            tracer.read_register(s_16_3 as isize, value);
            value
        };
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 2u16);
        // D s_16_6: cmp-eq s_16_2 s_16_5
        let s_16_6: bool = ((s_16_2) == (s_16_5));
        // D s_16_7: write-var gs#30326 <= s_16_6
        fn_state.gs_30326 = s_16_6;
        // N s_16_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#30326:u8
        let s_17_0: bool = fn_state.gs_30326;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_18_1: write-var cache.6 <= s_18_0
        fn_state.cache._6 = s_18_0;
        // N s_18_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var cache.6 <= s_19_0
        fn_state.cache._6 = s_19_0;
        // C s_19_2: const #() : ()
        let s_19_2: () = ();
        // S s_19_3: call VMID_read(s_19_2)
        let s_19_3: u16 = VMID_read(state, tracer, s_19_2);
        // D s_19_4: write-var cache.16 <= s_19_3
        fn_state.cache._16 = s_19_3;
        // N s_19_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#30326 <= s_20_0
        fn_state.gs_30326 = s_20_0;
        // N s_20_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var opscope:u32
        let s_21_0: u32 = fn_state.opscope;
        // C s_21_1: const #8u : u32
        let s_21_1: u32 = 8;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // N s_21_3: branch s_21_2 b36 b22
        if s_21_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var opscope:u32
        let s_22_0: u32 = fn_state.opscope;
        // C s_22_1: const #7u : u32
        let s_22_1: u32 = 7;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // N s_22_3: branch s_22_2 b35 b23
        if s_22_2 {
            return block_35(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#30341 <= s_23_0
        fn_state.gs_30341 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#30341:u8
        let s_24_0: bool = fn_state.gs_30341;
        // N s_24_1: branch s_24_0 b34 b25
        if s_24_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#30342 <= s_25_0
        fn_state.gs_30342 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#30342:u8
        let s_26_0: bool = fn_state.gs_30342;
        // N s_26_1: branch s_26_0 b33 b27
        if s_26_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#30343 <= s_27_0
        fn_state.gs_30343 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#30343:u8
        let s_28_0: bool = fn_state.gs_30343;
        // D s_28_1: write-var gs#30344 <= s_28_0
        fn_state.gs_30344 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#30344:u8
        let s_29_0: bool = fn_state.gs_30344;
        // N s_29_1: branch s_29_0 b32 b30
        if s_29_0 {
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
        // C s_30_0: const #0u : u32
        let s_30_0: u32 = 0;
        // D s_30_1: write-var cache.13 <= s_30_0
        fn_state.cache._13 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #64s : i
        let s_31_0: i128 = 64;
        // D s_31_1: read-var regval:u32
        let s_31_1: u32 = fn_state.regval;
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 32u16);
        // D s_31_3: bits-cast zx s_31_2 -> bv length s_31_0
        let s_31_3: Bits = s_31_2.zero_extend(s_31_0);
        // D s_31_4: cast reint s_31_3 -> u64
        let s_31_4: u64 = (s_31_3.value() as u64);
        // D s_31_5: write-var cache.10 <= s_31_4
        fn_state.cache._10 = s_31_4;
        // D s_31_6: read-var cache:struct
        let s_31_6: ProductType8ae001a7cc8b5154 = fn_state.cache;
        // D s_31_7: call CACHE_OP(s_31_6)
        let s_31_7: () = CACHE_OP(state, tracer, s_31_6);
        // N s_31_8: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u32
        let s_32_0: u32 = 1;
        // D s_32_1: write-var cache.13 <= s_32_0
        fn_state.cache._13 = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call HCR_read(s_33_0)
        let s_33_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_33_0);
        // S s_33_2: call _get_HCR_Type_FB(s_33_1)
        let s_33_2: bool = u_get_HCR_Type_FB(state, tracer, s_33_1);
        // S s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #1u : u8
        let s_33_4: bool = true;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // S s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // D s_33_7: write-var gs#30343 <= s_33_6
        fn_state.gs_30343 = s_33_6;
        // N s_33_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call EL2Enabled(s_34_0)
        let s_34_1: bool = EL2Enabled(state, tracer, s_34_0);
        // D s_34_2: write-var gs#30342 <= s_34_1
        fn_state.gs_30342 = s_34_1;
        // N s_34_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #16975u : u32
        let s_35_0: u32 = 16975;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: cast zx s_35_1 -> bv
        let s_35_2: Bits = Bits::new(s_35_1 as u128, 2u16);
        // C s_35_3: const #440u : u32
        let s_35_3: u32 = 440;
        // D s_35_4: read-reg s_35_3:u8
        let s_35_4: u8 = {
            let value = state.read_register::<u8>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // D s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 2u16);
        // D s_35_6: cmp-eq s_35_2 s_35_5
        let s_35_6: bool = ((s_35_2) == (s_35_5));
        // D s_35_7: write-var gs#30341 <= s_35_6
        fn_state.gs_30341 = s_35_6;
        // N s_35_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#30344 <= s_36_0
        fn_state.gs_30344 = s_36_0;
        // N s_36_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#30321 <= s_37_0
        fn_state.gs_30321 = s_37_0;
        // N s_37_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
