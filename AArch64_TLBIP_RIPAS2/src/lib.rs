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
use TLBIPRange::*;
use TLBI::*;
use Broadcast::*;
use Unreachable::*;
use common::*;
pub fn AArch64_TLBIP_RIPAS2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    security: u32,
    regime: u32,
    vmid: u16,
    shareability: u32,
    level: u32,
    attr: u32,
    Xt: u128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        ga_20923: ProductType37abbcb1894e7c56,
        gs_27122: bool,
        gs_27123: bool,
        security: u32,
        regime: u32,
        vmid: u16,
        shareability: u32,
        level: u32,
        attr: u32,
        Xt: u128,
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
        // N s_0_7: branch s_0_6 b20 b1
        if s_0_6 {
            return block_20(state, tracer, fn_state);
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
        // N s_1_7: branch s_1_6 b19 b2
        if s_1_6 {
            return block_19(state, tracer, fn_state);
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
        // D s_2_7: write-var gs#27122 <= s_2_6
        fn_state.gs_27122 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#27122:u8
        let s_3_0: bool = fn_state.gs_27122;
        // D s_3_1: write-var gs#27123 <= s_3_0
        fn_state.gs_27123 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#27123:u8
        let s_4_0: bool = fn_state.gs_27123;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #16u : u32
        let s_4_2: u32 = 16;
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
        // D s_4_18: read-var r.13:struct
        let s_4_18: u8 = fn_state.r._13;
        // C s_4_19: const #0s : i
        let s_4_19: i128 = 0;
        // D s_4_20: cast zx s_4_18 -> bv
        let s_4_20: Bits = Bits::new(s_4_18 as u128, 4u16);
        // C s_4_21: const #1s : i64
        let s_4_21: i64 = 1;
        // C s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (i128::try_from(s_4_21).unwrap());
        // C s_4_23: const #1s : i
        let s_4_23: i128 = 1;
        // C s_4_24: add s_4_23 s_4_22
        let s_4_24: i128 = (s_4_23 + s_4_22);
        // D s_4_25: bit-extract s_4_20 s_4_19 s_4_24
        let s_4_25: Bits = (Bits::new(
            ((s_4_20) >> (s_4_19)).value(),
            u16::try_from(s_4_24).unwrap(),
        ));
        // D s_4_26: cast reint s_4_25 -> u8
        let s_4_26: u8 = (s_4_25.value() as u8);
        // D s_4_27: cast zx s_4_26 -> bv
        let s_4_27: Bits = Bits::new(s_4_26 as u128, 2u16);
        // C s_4_28: const #0u : u8
        let s_4_28: u8 = 0;
        // C s_4_29: cast zx s_4_28 -> bv
        let s_4_29: Bits = Bits::new(s_4_28 as u128, 2u16);
        // D s_4_30: cmp-eq s_4_27 s_4_29
        let s_4_30: bool = ((s_4_27) == (s_4_29));
        // D s_4_31: write-var r.4 <= s_4_30
        fn_state.r._4 = s_4_30;
        // C s_4_32: const #1u : u8
        let s_4_32: bool = true;
        // D s_4_33: write-var r.3 <= s_4_32
        fn_state.r._3 = s_4_32;
        // D s_4_34: read-var regime:u32
        let s_4_34: u32 = fn_state.regime;
        // D s_4_35: read-var Xt:u128
        let s_4_35: u128 = fn_state.Xt;
        // D s_4_36: call TLBIPRange(s_4_34, s_4_35)
        let s_4_36: ProductType37abbcb1894e7c56 = TLBIPRange(
            state,
            tracer,
            s_4_34,
            s_4_35,
        );
        // D s_4_37: write-var ga#20923 <= s_4_36
        fn_state.ga_20923 = s_4_36;
        // D s_4_38: read-var ga#20923.0:struct
        let s_4_38: bool = fn_state.ga_20923._0;
        // D s_4_39: read-var ga#20923.1:struct
        let s_4_39: u8 = fn_state.ga_20923._1;
        // D s_4_40: read-var ga#20923.2:struct
        let s_4_40: u64 = fn_state.ga_20923._2;
        // D s_4_41: read-var ga#20923.3:struct
        let s_4_41: u64 = fn_state.ga_20923._3;
        // D s_4_42: write-var r.12 <= s_4_39
        fn_state.r._12 = s_4_39;
        // D s_4_43: write-var r.0 <= s_4_40
        fn_state.r._0 = s_4_40;
        // D s_4_44: write-var r.5 <= s_4_41
        fn_state.r._5 = s_4_41;
        // D s_4_45: not s_4_38
        let s_4_45: bool = !s_4_38;
        // N s_4_46: branch s_4_45 b18 b5
        if s_4_45 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u32
        let s_5_0: u32 = 0;
        // D s_5_1: read-var security:u32
        let s_5_1: u32 = fn_state.security;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b10 b6
        if s_5_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u32
        let s_6_0: u32 = 0;
        // D s_6_1: write-var r.7 <= s_6_0
        fn_state.r._7 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r:struct
        let s_7_0: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_7_1: read-var shareability:u32
        let s_7_1: u32 = fn_state.shareability;
        // D s_7_2: call TLBI(s_7_0, s_7_1)
        let s_7_2: () = TLBI(state, tracer, s_7_0, s_7_1);
        // D s_7_3: read-var shareability:u32
        let s_7_3: u32 = fn_state.shareability;
        // C s_7_4: const #0u : u32
        let s_7_4: u32 = 0;
        // D s_7_5: cmp-eq s_7_3 s_7_4
        let s_7_5: bool = ((s_7_3) == (s_7_4));
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
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
        // D s_9_0: read-var shareability:u32
        let s_9_0: u32 = fn_state.shareability;
        // D s_9_1: read-var r:struct
        let s_9_1: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_9_2: call Broadcast(s_9_0, s_9_1)
        let s_9_2: () = Broadcast(state, tracer, s_9_0, s_9_1);
        // N s_9_3: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #3u : u32
        let s_10_0: u32 = 3;
        // D s_10_1: read-var security:u32
        let s_10_1: u32 = fn_state.security;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b15 b11
        if s_10_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #63s : i
        let s_11_0: i128 = 63;
        // D s_11_1: read-var Xt:u128
        let s_11_1: u128 = fn_state.Xt;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 128u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #1u : u8
        let s_11_19: bool = true;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // N s_11_22: branch s_11_21 b14 b12
        if s_11_21 {
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
        // C s_12_0: const #1u : u32
        let s_12_0: u32 = 1;
        // D s_12_1: write-var r.7 <= s_12_0
        fn_state.r._7 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u32
        let s_14_0: u32 = 0;
        // D s_14_1: write-var r.7 <= s_14_0
        fn_state.r._7 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2u : u32
        let s_15_0: u32 = 2;
        // D s_15_1: read-var security:u32
        let s_15_1: u32 = fn_state.security;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b17 b16
        if s_15_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #3u : u32
        let s_16_0: u32 = 3;
        // D s_16_1: write-var r.7 <= s_16_0
        fn_state.r._7 = s_16_0;
        // N s_16_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call Unreachable(s_17_0)
        let s_17_1: () = Unreachable(state, tracer, s_17_0);
        // N s_17_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#27122 <= s_19_0
        fn_state.gs_27122 = s_19_0;
        // N s_19_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#27123 <= s_20_0
        fn_state.gs_27123 = s_20_0;
        // N s_20_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
