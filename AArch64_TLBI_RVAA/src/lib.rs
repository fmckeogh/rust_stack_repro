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
use Broadcast::*;
use TLBIRange::*;
use TLBI::*;
use common::*;
pub fn AArch64_TLBI_RVAA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    security: u32,
    regime: u32,
    vmid: u16,
    shareability: u32,
    level: u32,
    attr: u32,
    Xt: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        gs_26985: bool,
        ga_20860: ProductType37abbcb1894e7c56,
        gs_26986: bool,
        security: u32,
        regime: u32,
        vmid: u16,
        shareability: u32,
        level: u32,
        attr: u32,
        Xt: u64,
    }
    let fn_state = FunctionState {
        security,
        regime,
        vmid,
        shareability,
        level,
        attr,
        Xt,
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
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b10 b1
        if s_0_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #432u : u32
        let s_1_3: u32 = 432;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b9 b2
        if s_1_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16975u : u32
        let s_2_0: u32 = 16975;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #440u : u32
        let s_2_3: u32 = 440;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // D s_2_7: write-var gs#26985 <= s_2_6
        fn_state.gs_26985 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#26985:u8
        let s_3_0: bool = fn_state.gs_26985;
        // D s_3_1: write-var gs#26986 <= s_3_0
        fn_state.gs_26986 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#26986:u8
        let s_4_0: bool = fn_state.gs_26986;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #18u : u32
        let s_4_2: u32 = 18;
        // D s_4_3: write-var r.9 <= s_4_2
        fn_state.r._9 = s_4_2;
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // D s_4_5: write-var r.6 <= s_4_4
        fn_state.r._6 = s_4_4;
        // D s_4_6: read-var security:u32
        let s_4_6: u32 = fn_state.security;
        // D s_4_7: write-var r.11 <= s_4_6
        fn_state.r._11 = s_4_6;
        // D s_4_8: read-var regime:u32
        let s_4_8: u32 = fn_state.regime;
        // D s_4_9: write-var r.10 <= s_4_8
        fn_state.r._10 = s_4_8;
        // D s_4_10: read-var vmid:u16
        let s_4_10: u16 = fn_state.vmid;
        // D s_4_11: write-var r.14 <= s_4_10
        fn_state.r._14 = s_4_10;
        // D s_4_12: read-var level:u32
        let s_4_12: u32 = fn_state.level;
        // D s_4_13: write-var r.8 <= s_4_12
        fn_state.r._8 = s_4_12;
        // D s_4_14: read-var attr:u32
        let s_4_14: u32 = fn_state.attr;
        // D s_4_15: write-var r.2 <= s_4_14
        fn_state.r._2 = s_4_14;
        // D s_4_16: read-var r:struct
        let s_4_16: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_4_17: write-var r <= s_4_16
        fn_state.r = s_4_16;
        // C s_4_18: const #1u : u8
        let s_4_18: bool = true;
        // D s_4_19: write-var r.4 <= s_4_18
        fn_state.r._4 = s_4_18;
        // D s_4_20: read-var r.13:struct
        let s_4_20: u8 = fn_state.r._13;
        // C s_4_21: const #0s : i
        let s_4_21: i128 = 0;
        // D s_4_22: cast zx s_4_20 -> bv
        let s_4_22: Bits = Bits::new(s_4_20 as u128, 4u16);
        // C s_4_23: const #1s : i64
        let s_4_23: i64 = 1;
        // C s_4_24: cast zx s_4_23 -> i
        let s_4_24: i128 = (i128::try_from(s_4_23).unwrap());
        // C s_4_25: const #1s : i
        let s_4_25: i128 = 1;
        // C s_4_26: add s_4_25 s_4_24
        let s_4_26: i128 = (s_4_25 + s_4_24);
        // D s_4_27: bit-extract s_4_22 s_4_21 s_4_26
        let s_4_27: Bits = (Bits::new(
            ((s_4_22) >> (s_4_21)).value(),
            u16::try_from(s_4_26).unwrap(),
        ));
        // D s_4_28: cast reint s_4_27 -> u8
        let s_4_28: u8 = (s_4_27.value() as u8);
        // D s_4_29: cast zx s_4_28 -> bv
        let s_4_29: Bits = Bits::new(s_4_28 as u128, 2u16);
        // C s_4_30: const #0u : u8
        let s_4_30: u8 = 0;
        // C s_4_31: cast zx s_4_30 -> bv
        let s_4_31: Bits = Bits::new(s_4_30 as u128, 2u16);
        // D s_4_32: cmp-eq s_4_29 s_4_31
        let s_4_32: bool = ((s_4_29) == (s_4_31));
        // D s_4_33: write-var r.3 <= s_4_32
        fn_state.r._3 = s_4_32;
        // D s_4_34: read-var regime:u32
        let s_4_34: u32 = fn_state.regime;
        // D s_4_35: read-var Xt:u64
        let s_4_35: u64 = fn_state.Xt;
        // D s_4_36: call TLBIRange(s_4_34, s_4_35)
        let s_4_36: ProductType37abbcb1894e7c56 = TLBIRange(
            state,
            tracer,
            s_4_34,
            s_4_35,
        );
        // D s_4_37: write-var ga#20860 <= s_4_36
        fn_state.ga_20860 = s_4_36;
        // D s_4_38: read-var ga#20860.0:struct
        let s_4_38: bool = fn_state.ga_20860._0;
        // D s_4_39: read-var ga#20860.1:struct
        let s_4_39: u8 = fn_state.ga_20860._1;
        // D s_4_40: read-var ga#20860.2:struct
        let s_4_40: u64 = fn_state.ga_20860._2;
        // D s_4_41: read-var ga#20860.3:struct
        let s_4_41: u64 = fn_state.ga_20860._3;
        // D s_4_42: write-var r.12 <= s_4_39
        fn_state.r._12 = s_4_39;
        // D s_4_43: write-var r.0 <= s_4_40
        fn_state.r._0 = s_4_40;
        // D s_4_44: write-var r.5 <= s_4_41
        fn_state.r._5 = s_4_41;
        // D s_4_45: not s_4_38
        let s_4_45: bool = !s_4_38;
        // N s_4_46: branch s_4_45 b8 b5
        if s_4_45 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:struct
        let s_5_0: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_5_1: read-var shareability:u32
        let s_5_1: u32 = fn_state.shareability;
        // D s_5_2: call TLBI(s_5_0, s_5_1)
        let s_5_2: () = TLBI(state, tracer, s_5_0, s_5_1);
        // D s_5_3: read-var shareability:u32
        let s_5_3: u32 = fn_state.shareability;
        // C s_5_4: const #0u : u32
        let s_5_4: u32 = 0;
        // D s_5_5: cmp-eq s_5_3 s_5_4
        let s_5_5: bool = ((s_5_3) == (s_5_4));
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
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
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var shareability:u32
        let s_7_0: u32 = fn_state.shareability;
        // D s_7_1: read-var r:struct
        let s_7_1: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_7_2: call Broadcast(s_7_0, s_7_1)
        let s_7_2: () = Broadcast(state, tracer, s_7_0, s_7_1);
        // N s_7_3: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#26985 <= s_9_0
        fn_state.gs_26985 = s_9_0;
        // N s_9_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#26986 <= s_10_0
        fn_state.gs_26986 = s_10_0;
        // N s_10_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
