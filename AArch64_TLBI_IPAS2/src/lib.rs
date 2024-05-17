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
use TLBI::*;
use Broadcast::*;
use place_subrange::*;
use Unreachable::*;
use common::*;
pub fn AArch64_TLBI_IPAS2<T: Tracer>(
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
        gs_26661: bool,
        gs_26669: bool,
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
        // N s_0_7: branch s_0_6 b18 b1
        if s_0_6 {
            return block_18(state, tracer, fn_state);
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
        // D s_1_7: write-var gs#26661 <= s_1_6
        fn_state.gs_26661 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#26661:u8
        let s_2_0: bool = fn_state.gs_26661;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #8u : u32
        let s_2_2: u32 = 8;
        // D s_2_3: write-var r.9 <= s_2_2
        fn_state.r._9 = s_2_2;
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // D s_2_5: write-var r.6 <= s_2_4
        fn_state.r._6 = s_2_4;
        // D s_2_6: read-var security:u32
        let s_2_6: u32 = fn_state.security;
        // D s_2_7: write-var r.11 <= s_2_6
        fn_state.r._11 = s_2_6;
        // D s_2_8: read-var regime:u32
        let s_2_8: u32 = fn_state.regime;
        // D s_2_9: write-var r.10 <= s_2_8
        fn_state.r._10 = s_2_8;
        // D s_2_10: read-var vmid:u16
        let s_2_10: u16 = fn_state.vmid;
        // D s_2_11: write-var r.14 <= s_2_10
        fn_state.r._14 = s_2_10;
        // D s_2_12: read-var level:u32
        let s_2_12: u32 = fn_state.level;
        // D s_2_13: write-var r.8 <= s_2_12
        fn_state.r._8 = s_2_12;
        // D s_2_14: read-var attr:u32
        let s_2_14: u32 = fn_state.attr;
        // D s_2_15: write-var r.2 <= s_2_14
        fn_state.r._2 = s_2_14;
        // C s_2_16: const #44s : i
        let s_2_16: i128 = 44;
        // D s_2_17: read-var Xt:u64
        let s_2_17: u64 = fn_state.Xt;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 64u16);
        // C s_2_19: const #1s : i64
        let s_2_19: i64 = 1;
        // C s_2_20: cast zx s_2_19 -> i
        let s_2_20: i128 = (i128::try_from(s_2_19).unwrap());
        // C s_2_21: const #3s : i
        let s_2_21: i128 = 3;
        // C s_2_22: add s_2_21 s_2_20
        let s_2_22: i128 = (s_2_21 + s_2_20);
        // D s_2_23: bit-extract s_2_18 s_2_16 s_2_22
        let s_2_23: Bits = (Bits::new(
            ((s_2_18) >> (s_2_16)).value(),
            u16::try_from(s_2_22).unwrap(),
        ));
        // D s_2_24: cast reint s_2_23 -> u8
        let s_2_24: u8 = (s_2_23.value() as u8);
        // D s_2_25: write-var r.13 <= s_2_24
        fn_state.r._13 = s_2_24;
        // C s_2_26: const #64s : i
        let s_2_26: i128 = 64;
        // C s_2_27: const #39s : i
        let s_2_27: i128 = 39;
        // C s_2_28: const #0s : i
        let s_2_28: i128 = 0;
        // C s_2_29: const #12s : i
        let s_2_29: i128 = 12;
        // D s_2_30: read-var Xt:u64
        let s_2_30: u64 = fn_state.Xt;
        // D s_2_31: cast zx s_2_30 -> bv
        let s_2_31: Bits = Bits::new(s_2_30 as u128, 64u16);
        // D s_2_32: call place_subrange(s_2_26, s_2_31, s_2_27, s_2_28, s_2_29)
        let s_2_32: Bits = place_subrange(
            state,
            tracer,
            s_2_26,
            s_2_31,
            s_2_27,
            s_2_28,
            s_2_29,
        );
        // D s_2_33: cast reint s_2_32 -> u64
        let s_2_33: u64 = (s_2_32.value() as u64);
        // D s_2_34: write-var r.0 <= s_2_33
        fn_state.r._0 = s_2_33;
        // C s_2_35: const #1u : u8
        let s_2_35: bool = true;
        // D s_2_36: write-var r.4 <= s_2_35
        fn_state.r._4 = s_2_35;
        // D s_2_37: read-var r.13:struct
        let s_2_37: u8 = fn_state.r._13;
        // C s_2_38: const #2s : i
        let s_2_38: i128 = 2;
        // D s_2_39: cast zx s_2_37 -> bv
        let s_2_39: Bits = Bits::new(s_2_37 as u128, 4u16);
        // C s_2_40: const #1s : i64
        let s_2_40: i64 = 1;
        // C s_2_41: cast zx s_2_40 -> i
        let s_2_41: i128 = (i128::try_from(s_2_40).unwrap());
        // C s_2_42: const #1s : i
        let s_2_42: i128 = 1;
        // C s_2_43: add s_2_42 s_2_41
        let s_2_43: i128 = (s_2_42 + s_2_41);
        // D s_2_44: bit-extract s_2_39 s_2_38 s_2_43
        let s_2_44: Bits = (Bits::new(
            ((s_2_39) >> (s_2_38)).value(),
            u16::try_from(s_2_43).unwrap(),
        ));
        // D s_2_45: cast reint s_2_44 -> u8
        let s_2_45: u8 = (s_2_44.value() as u8);
        // D s_2_46: cast zx s_2_45 -> bv
        let s_2_46: Bits = Bits::new(s_2_45 as u128, 2u16);
        // C s_2_47: const #0u : u8
        let s_2_47: u8 = 0;
        // C s_2_48: cast zx s_2_47 -> bv
        let s_2_48: Bits = Bits::new(s_2_47 as u128, 2u16);
        // D s_2_49: cmp-eq s_2_46 s_2_48
        let s_2_49: bool = ((s_2_46) == (s_2_48));
        // D s_2_50: not s_2_49
        let s_2_50: bool = !s_2_49;
        // N s_2_51: branch s_2_50 b17 b3
        if s_2_50 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#26669 <= s_3_0
        fn_state.gs_26669 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#26669:u8
        let s_4_0: bool = fn_state.gs_26669;
        // D s_4_1: write-var r.3 <= s_4_0
        fn_state.r._3 = s_4_0;
        // C s_4_2: const #0u : u32
        let s_4_2: u32 = 0;
        // D s_4_3: read-var security:u32
        let s_4_3: u32 = fn_state.security;
        // D s_4_4: cmp-eq s_4_2 s_4_3
        let s_4_4: bool = ((s_4_2) == (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b9 b5
        if s_4_5 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_1: write-var r.7 <= s_5_0
        fn_state.r._7 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:struct
        let s_6_0: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_6_1: read-var shareability:u32
        let s_6_1: u32 = fn_state.shareability;
        // D s_6_2: call TLBI(s_6_0, s_6_1)
        let s_6_2: () = TLBI(state, tracer, s_6_0, s_6_1);
        // D s_6_3: read-var shareability:u32
        let s_6_3: u32 = fn_state.shareability;
        // C s_6_4: const #0u : u32
        let s_6_4: u32 = 0;
        // D s_6_5: cmp-eq s_6_3 s_6_4
        let s_6_5: bool = ((s_6_3) == (s_6_4));
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
        // D s_8_0: read-var shareability:u32
        let s_8_0: u32 = fn_state.shareability;
        // D s_8_1: read-var r:struct
        let s_8_1: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_8_2: call Broadcast(s_8_0, s_8_1)
        let s_8_2: () = Broadcast(state, tracer, s_8_0, s_8_1);
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #3u : u32
        let s_9_0: u32 = 3;
        // D s_9_1: read-var security:u32
        let s_9_1: u32 = fn_state.security;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b14 b10
        if s_9_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #63s : i
        let s_10_0: i128 = 63;
        // D s_10_1: read-var Xt:u64
        let s_10_1: u64 = fn_state.Xt;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 64u16);
        // C s_10_3: const #1u : u64
        let s_10_3: u64 = 1;
        // D s_10_4: bit-extract s_10_2 s_10_0 s_10_3
        let s_10_4: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_3).unwrap(),
        ));
        // D s_10_5: cast reint s_10_4 -> u8
        let s_10_5: bool = ((s_10_4.value()) != 0);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // C s_10_7: const #0u : u64
        let s_10_7: u64 = 0;
        // D s_10_8: cast zx s_10_5 -> u64
        let s_10_8: u64 = (s_10_5 as u64);
        // C s_10_9: const #1u : u64
        let s_10_9: u64 = 1;
        // D s_10_10: and s_10_8 s_10_9
        let s_10_10: u64 = ((s_10_8) & (s_10_9));
        // D s_10_11: cmp-eq s_10_10 s_10_9
        let s_10_11: bool = ((s_10_10) == (s_10_9));
        // D s_10_12: lsl s_10_8 s_10_6
        let s_10_12: u64 = s_10_8 << s_10_6;
        // D s_10_13: or s_10_7 s_10_12
        let s_10_13: u64 = ((s_10_7) | (s_10_12));
        // D s_10_14: cmpl s_10_12
        let s_10_14: u64 = !s_10_12;
        // D s_10_15: and s_10_7 s_10_14
        let s_10_15: u64 = ((s_10_7) & (s_10_14));
        // D s_10_16: select s_10_11 s_10_13 s_10_15
        let s_10_16: u64 = if s_10_11 { s_10_13 } else { s_10_15 };
        // D s_10_17: cast trunc s_10_16 -> u8
        let s_10_17: bool = ((s_10_16) != 0);
        // D s_10_18: cast zx s_10_17 -> bv
        let s_10_18: Bits = Bits::new(s_10_17 as u128, 1u16);
        // C s_10_19: const #1u : u8
        let s_10_19: bool = true;
        // C s_10_20: cast zx s_10_19 -> bv
        let s_10_20: Bits = Bits::new(s_10_19 as u128, 1u16);
        // D s_10_21: cmp-eq s_10_18 s_10_20
        let s_10_21: bool = ((s_10_18) == (s_10_20));
        // N s_10_22: branch s_10_21 b13 b11
        if s_10_21 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u32
        let s_11_0: u32 = 1;
        // D s_11_1: write-var r.7 <= s_11_0
        fn_state.r._7 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u32
        let s_13_0: u32 = 0;
        // D s_13_1: write-var r.7 <= s_13_0
        fn_state.r._7 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #2u : u32
        let s_14_0: u32 = 2;
        // D s_14_1: read-var security:u32
        let s_14_1: u32 = fn_state.security;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b16 b15
        if s_14_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #3u : u32
        let s_15_0: u32 = 3;
        // D s_15_1: write-var r.7 <= s_15_0
        fn_state.r._7 = s_15_0;
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call Unreachable(s_16_0)
        let s_16_1: () = Unreachable(state, tracer, s_16_0);
        // N s_16_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#26669 <= s_17_0
        fn_state.gs_26669 = s_17_0;
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#26661 <= s_18_0
        fn_state.gs_26661 = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
