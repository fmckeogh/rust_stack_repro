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
use TLBI::*;
use Zeros::*;
use common::*;
pub fn AArch32_TLBI_IPAS2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    security: u32,
    regime: u32,
    vmid: u16,
    shareability: u32,
    level: u32,
    attr: u32,
    Rt: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        gs_32472: bool,
        security: u32,
        regime: u32,
        vmid: u16,
        shareability: u32,
        level: u32,
        attr: u32,
        Rt: u32,
    }
    let fn_state = FunctionState {
        security,
        regime,
        vmid,
        shareability,
        level,
        attr,
        Rt,
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
        // N s_0_7: branch s_0_6 b5 b1
        if s_0_6 {
            return block_5(state, tracer, fn_state);
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
        // D s_1_7: write-var gs#32472 <= s_1_6
        fn_state.gs_32472 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#32472:u8
        let s_2_0: bool = fn_state.gs_32472;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var security:u32
        let s_2_2: u32 = fn_state.security;
        // C s_2_3: const #0u : u32
        let s_2_3: u32 = 0;
        // D s_2_4: cmp-eq s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) == (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // C s_2_6: const #8u : u32
        let s_2_6: u32 = 8;
        // D s_2_7: write-var r.9 <= s_2_6
        fn_state.r._9 = s_2_6;
        // C s_2_8: const #0u : u8
        let s_2_8: bool = false;
        // D s_2_9: write-var r.6 <= s_2_8
        fn_state.r._6 = s_2_8;
        // D s_2_10: read-var security:u32
        let s_2_10: u32 = fn_state.security;
        // D s_2_11: write-var r.11 <= s_2_10
        fn_state.r._11 = s_2_10;
        // D s_2_12: read-var regime:u32
        let s_2_12: u32 = fn_state.regime;
        // D s_2_13: write-var r.10 <= s_2_12
        fn_state.r._10 = s_2_12;
        // D s_2_14: read-var vmid:u16
        let s_2_14: u16 = fn_state.vmid;
        // D s_2_15: write-var r.14 <= s_2_14
        fn_state.r._14 = s_2_14;
        // D s_2_16: read-var level:u32
        let s_2_16: u32 = fn_state.level;
        // D s_2_17: write-var r.8 <= s_2_16
        fn_state.r._8 = s_2_16;
        // D s_2_18: read-var attr:u32
        let s_2_18: u32 = fn_state.attr;
        // D s_2_19: write-var r.2 <= s_2_18
        fn_state.r._2 = s_2_18;
        // C s_2_20: const #24s : i
        let s_2_20: i128 = 24;
        // S s_2_21: call Zeros(s_2_20)
        let s_2_21: Bits = Zeros(state, tracer, s_2_20);
        // S s_2_22: cast reint s_2_21 -> u24
        let s_2_22: u32 = (s_2_21.value() as u32);
        // C s_2_23: const #0s : i
        let s_2_23: i128 = 0;
        // D s_2_24: read-var Rt:u32
        let s_2_24: u32 = fn_state.Rt;
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 32u16);
        // C s_2_26: const #1s : i64
        let s_2_26: i64 = 1;
        // C s_2_27: cast zx s_2_26 -> i
        let s_2_27: i128 = (i128::try_from(s_2_26).unwrap());
        // C s_2_28: const #27s : i
        let s_2_28: i128 = 27;
        // C s_2_29: add s_2_28 s_2_27
        let s_2_29: i128 = (s_2_28 + s_2_27);
        // D s_2_30: bit-extract s_2_25 s_2_23 s_2_29
        let s_2_30: Bits = (Bits::new(
            ((s_2_25) >> (s_2_23)).value(),
            u16::try_from(s_2_29).unwrap(),
        ));
        // D s_2_31: cast reint s_2_30 -> u28
        let s_2_31: u32 = (s_2_30.value() as u32);
        // S s_2_32: cast zx s_2_22 -> bv
        let s_2_32: Bits = Bits::new(s_2_22 as u128, 24u16);
        // D s_2_33: cast zx s_2_31 -> bv
        let s_2_33: Bits = Bits::new(s_2_31 as u128, 28u16);
        // S s_2_34: cast reint s_2_32 -> u128
        let s_2_34: u128 = (s_2_32.value() as u128);
        // D s_2_35: size-of s_2_32
        let s_2_35: u16 = s_2_32.length();
        // D s_2_36: cast reint s_2_33 -> u128
        let s_2_36: u128 = (s_2_33.value() as u128);
        // D s_2_37: size-of s_2_33
        let s_2_37: u16 = s_2_33.length();
        // D s_2_38: lsl s_2_34 s_2_37
        let s_2_38: u128 = s_2_34 << s_2_37;
        // D s_2_39: or s_2_38 s_2_36
        let s_2_39: u128 = ((s_2_38) | (s_2_36));
        // D s_2_40: add s_2_35 s_2_37
        let s_2_40: u16 = (s_2_35 + s_2_37);
        // D s_2_41: create-bits s_2_39 s_2_40
        let s_2_41: Bits = Bits::new(s_2_39, s_2_40);
        // D s_2_42: cast reint s_2_41 -> u52
        let s_2_42: u64 = (s_2_41.value() as u64);
        // C s_2_43: const #12s : i
        let s_2_43: i128 = 12;
        // S s_2_44: call Zeros(s_2_43)
        let s_2_44: Bits = Zeros(state, tracer, s_2_43);
        // S s_2_45: cast reint s_2_44 -> u12
        let s_2_45: u16 = (s_2_44.value() as u16);
        // D s_2_46: cast zx s_2_42 -> bv
        let s_2_46: Bits = Bits::new(s_2_42 as u128, 52u16);
        // S s_2_47: cast zx s_2_45 -> bv
        let s_2_47: Bits = Bits::new(s_2_45 as u128, 12u16);
        // D s_2_48: cast reint s_2_46 -> u128
        let s_2_48: u128 = (s_2_46.value() as u128);
        // D s_2_49: size-of s_2_46
        let s_2_49: u16 = s_2_46.length();
        // S s_2_50: cast reint s_2_47 -> u128
        let s_2_50: u128 = (s_2_47.value() as u128);
        // D s_2_51: size-of s_2_47
        let s_2_51: u16 = s_2_47.length();
        // D s_2_52: lsl s_2_48 s_2_51
        let s_2_52: u128 = s_2_48 << s_2_51;
        // D s_2_53: or s_2_52 s_2_50
        let s_2_53: u128 = ((s_2_52) | (s_2_50));
        // D s_2_54: add s_2_49 s_2_51
        let s_2_54: u16 = (s_2_49 + s_2_51);
        // D s_2_55: create-bits s_2_53 s_2_54
        let s_2_55: Bits = Bits::new(s_2_53, s_2_54);
        // D s_2_56: cast reint s_2_55 -> u64
        let s_2_56: u64 = (s_2_55.value() as u64);
        // D s_2_57: write-var r.0 <= s_2_56
        fn_state.r._0 = s_2_56;
        // C s_2_58: const #0u : u32
        let s_2_58: u32 = 0;
        // D s_2_59: write-var r.7 <= s_2_58
        fn_state.r._7 = s_2_58;
        // D s_2_60: read-var r:struct
        let s_2_60: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_2_61: read-var shareability:u32
        let s_2_61: u32 = fn_state.shareability;
        // D s_2_62: call TLBI(s_2_60, s_2_61)
        let s_2_62: () = TLBI(state, tracer, s_2_60, s_2_61);
        // D s_2_63: read-var shareability:u32
        let s_2_63: u32 = fn_state.shareability;
        // C s_2_64: const #0u : u32
        let s_2_64: u32 = 0;
        // D s_2_65: cmp-eq s_2_63 s_2_64
        let s_2_65: bool = ((s_2_63) == (s_2_64));
        // N s_2_66: branch s_2_65 b4 b3
        if s_2_65 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var shareability:u32
        let s_4_0: u32 = fn_state.shareability;
        // D s_4_1: read-var r:struct
        let s_4_1: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_4_2: call Broadcast(s_4_0, s_4_1)
        let s_4_2: () = Broadcast(state, tracer, s_4_0, s_4_1);
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#32472 <= s_5_0
        fn_state.gs_32472 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
