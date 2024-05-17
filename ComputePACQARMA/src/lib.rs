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
use PACSub::*;
use PACMult::*;
use PACInvSub::*;
use TweakShuffle::*;
use PACSub1::*;
use PACCellInvShuffle::*;
use PACCellShuffle::*;
use TweakInvShuffle::*;
use common::*;
pub fn ComputePACQARMA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    data: u64,
    modifier: u64,
    key0: u64,
    key1: u64,
    isqarma3: bool,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        roundkey: u64,
        iterations: i64,
        gs_14641: i64,
        modk0: u64,
        workingval: u64,
        gs_14656: i64,
        iterationsshadow_257: i64,
        u_385: i64,
        i: i64,
        runningmod: u64,
        data: u64,
        modifier: u64,
        key0: u64,
        key1: u64,
        isqarma3: bool,
    }
    let fn_state = FunctionState {
        data,
        modifier,
        key0,
        key1,
        isqarma3,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #2s : i64
        let s_0_0: i64 = 2;
        // D s_0_1: write-var iterations <= s_0_0
        fn_state.iterations = s_0_0;
        // C s_0_2: const #0s : i
        let s_0_2: i128 = 0;
        // C s_0_3: const #20560u : u32
        let s_0_3: u32 = 20560;
        // D s_0_4: read-reg s_0_3:[u64; 5]
        let s_0_4: [u64; 5usize] = {
            let value = state.read_register::<[u64; 5usize]>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // C s_0_5: const #0u : u64
        let s_0_5: u64 = 0;
        // D s_0_6: mutate-element s_0_4[s_0_2] <= s_0_5
        let s_0_6: [u64; 5usize] = {
            let mut local = s_0_4.clone();
            local[(s_0_2) as usize] = s_0_5;
            local
        };
        // D s_0_7: cast cvt s_0_6 -> [u64; 0]
        let s_0_7: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_0_6);
        // D s_0_8: cast cvt s_0_7 -> [u64; 5]
        let s_0_8: [u64; 5usize] = {
            let mut buf = [Default::default(); 5usize];
            buf.copy_from_slice(&s_0_7);
            buf
        };
        // C s_0_9: const #20560u : u32
        let s_0_9: u32 = 20560;
        // N s_0_10: write-reg s_0_9 <= s_0_8
        let s_0_10: () = {
            state.write_register::<[u64; 5usize]>(s_0_9 as isize, s_0_8);
            tracer.write_register(s_0_9 as isize, s_0_8);
        };
        // C s_0_11: const #1s : i
        let s_0_11: i128 = 1;
        // C s_0_12: const #20560u : u32
        let s_0_12: u32 = 20560;
        // D s_0_13: read-reg s_0_12:[u64; 5]
        let s_0_13: [u64; 5usize] = {
            let value = state.read_register::<[u64; 5usize]>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // C s_0_14: const #1376283091369227076u : u64
        let s_0_14: u64 = 1376283091369227076;
        // D s_0_15: mutate-element s_0_13[s_0_11] <= s_0_14
        let s_0_15: [u64; 5usize] = {
            let mut local = s_0_13.clone();
            local[(s_0_11) as usize] = s_0_14;
            local
        };
        // D s_0_16: cast cvt s_0_15 -> [u64; 0]
        let s_0_16: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_0_15);
        // D s_0_17: cast cvt s_0_16 -> [u64; 5]
        let s_0_17: [u64; 5usize] = {
            let mut buf = [Default::default(); 5usize];
            buf.copy_from_slice(&s_0_16);
            buf
        };
        // C s_0_18: const #20560u : u32
        let s_0_18: u32 = 20560;
        // N s_0_19: write-reg s_0_18 <= s_0_17
        let s_0_19: () = {
            state.write_register::<[u64; 5usize]>(s_0_18 as isize, s_0_17);
            tracer.write_register(s_0_18 as isize, s_0_17);
        };
        // C s_0_20: const #2s : i
        let s_0_20: i128 = 2;
        // C s_0_21: const #20560u : u32
        let s_0_21: u32 = 20560;
        // D s_0_22: read-reg s_0_21:[u64; 5]
        let s_0_22: [u64; 5usize] = {
            let value = state.read_register::<[u64; 5usize]>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // C s_0_23: const #11820040416388919760u : u64
        let s_0_23: u64 = 11820040416388919760;
        // D s_0_24: mutate-element s_0_22[s_0_20] <= s_0_23
        let s_0_24: [u64; 5usize] = {
            let mut local = s_0_22.clone();
            local[(s_0_20) as usize] = s_0_23;
            local
        };
        // D s_0_25: cast cvt s_0_24 -> [u64; 0]
        let s_0_25: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_0_24);
        // D s_0_26: cast cvt s_0_25 -> [u64; 5]
        let s_0_26: [u64; 5usize] = {
            let mut buf = [Default::default(); 5usize];
            buf.copy_from_slice(&s_0_25);
            buf
        };
        // C s_0_27: const #20560u : u32
        let s_0_27: u32 = 20560;
        // N s_0_28: write-reg s_0_27 <= s_0_26
        let s_0_28: () = {
            state.write_register::<[u64; 5usize]>(s_0_27 as isize, s_0_26);
            tracer.write_register(s_0_27 as isize, s_0_26);
        };
        // D s_0_29: read-var isqarma3:u8
        let s_0_29: bool = fn_state.isqarma3;
        // N s_0_30: branch s_0_29 b27 b1
        if s_0_29 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #4s : i64
        let s_1_0: i64 = 4;
        // D s_1_1: write-var iterations <= s_1_0
        fn_state.iterations = s_1_0;
        // C s_1_2: const #3s : i
        let s_1_2: i128 = 3;
        // C s_1_3: const #20560u : u32
        let s_1_3: u32 = 20560;
        // D s_1_4: read-reg s_1_3:[u64; 5]
        let s_1_4: [u64; 5usize] = {
            let value = state.read_register::<[u64; 5usize]>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // C s_1_5: const #589684135938649225u : u64
        let s_1_5: u64 = 589684135938649225;
        // D s_1_6: mutate-element s_1_4[s_1_2] <= s_1_5
        let s_1_6: [u64; 5usize] = {
            let mut local = s_1_4.clone();
            local[(s_1_2) as usize] = s_1_5;
            local
        };
        // D s_1_7: cast cvt s_1_6 -> [u64; 0]
        let s_1_7: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_1_6);
        // D s_1_8: cast cvt s_1_7 -> [u64; 5]
        let s_1_8: [u64; 5usize] = {
            let mut buf = [Default::default(); 5usize];
            buf.copy_from_slice(&s_1_7);
            buf
        };
        // C s_1_9: const #20560u : u32
        let s_1_9: u32 = 20560;
        // N s_1_10: write-reg s_1_9 <= s_1_8
        let s_1_10: () = {
            state.write_register::<[u64; 5usize]>(s_1_9 as isize, s_1_8);
            tracer.write_register(s_1_9 as isize, s_1_8);
        };
        // C s_1_11: const #4s : i
        let s_1_11: i128 = 4;
        // C s_1_12: const #20560u : u32
        let s_1_12: u32 = 20560;
        // D s_1_13: read-reg s_1_12:[u64; 5]
        let s_1_13: [u64; 5usize] = {
            let value = state.read_register::<[u64; 5usize]>(s_1_12 as isize);
            tracer.read_register(s_1_12 as isize, value);
            value
        };
        // C s_1_14: const #4983270260364809079u : u64
        let s_1_14: u64 = 4983270260364809079;
        // D s_1_15: mutate-element s_1_13[s_1_11] <= s_1_14
        let s_1_15: [u64; 5usize] = {
            let mut local = s_1_13.clone();
            local[(s_1_11) as usize] = s_1_14;
            local
        };
        // D s_1_16: cast cvt s_1_15 -> [u64; 0]
        let s_1_16: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_1_15);
        // D s_1_17: cast cvt s_1_16 -> [u64; 5]
        let s_1_17: [u64; 5usize] = {
            let mut buf = [Default::default(); 5usize];
            buf.copy_from_slice(&s_1_16);
            buf
        };
        // C s_1_18: const #20560u : u32
        let s_1_18: u32 = 20560;
        // N s_1_19: write-reg s_1_18 <= s_1_17
        let s_1_19: () = {
            state.write_register::<[u64; 5usize]>(s_1_18 as isize, s_1_17);
            tracer.write_register(s_1_18 as isize, s_1_17);
        };
        // N s_1_20: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var iterations:i64
        let s_2_0: i64 = fn_state.iterations;
        // D s_2_1: write-var iterationsshadow#257 <= s_2_0
        fn_state.iterationsshadow_257 = s_2_0;
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: read-var key0:u64
        let s_2_3: u64 = fn_state.key0;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 64u16);
        // C s_2_5: const #1u : u64
        let s_2_5: u64 = 1;
        // D s_2_6: bit-extract s_2_4 s_2_2 s_2_5
        let s_2_6: Bits = (Bits::new(
            ((s_2_4) >> (s_2_2)).value(),
            u16::try_from(s_2_5).unwrap(),
        ));
        // D s_2_7: cast reint s_2_6 -> u8
        let s_2_7: bool = ((s_2_6.value()) != 0);
        // C s_2_8: const #0s : i
        let s_2_8: i128 = 0;
        // C s_2_9: const #0u : u64
        let s_2_9: u64 = 0;
        // D s_2_10: cast zx s_2_7 -> u64
        let s_2_10: u64 = (s_2_7 as u64);
        // C s_2_11: const #1u : u64
        let s_2_11: u64 = 1;
        // D s_2_12: and s_2_10 s_2_11
        let s_2_12: u64 = ((s_2_10) & (s_2_11));
        // D s_2_13: cmp-eq s_2_12 s_2_11
        let s_2_13: bool = ((s_2_12) == (s_2_11));
        // D s_2_14: lsl s_2_10 s_2_8
        let s_2_14: u64 = s_2_10 << s_2_8;
        // D s_2_15: or s_2_9 s_2_14
        let s_2_15: u64 = ((s_2_9) | (s_2_14));
        // D s_2_16: cmpl s_2_14
        let s_2_16: u64 = !s_2_14;
        // D s_2_17: and s_2_9 s_2_16
        let s_2_17: u64 = ((s_2_9) & (s_2_16));
        // D s_2_18: select s_2_13 s_2_15 s_2_17
        let s_2_18: u64 = if s_2_13 { s_2_15 } else { s_2_17 };
        // D s_2_19: cast trunc s_2_18 -> u8
        let s_2_19: bool = ((s_2_18) != 0);
        // C s_2_20: const #2s : i
        let s_2_20: i128 = 2;
        // D s_2_21: read-var key0:u64
        let s_2_21: u64 = fn_state.key0;
        // D s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 64u16);
        // C s_2_23: const #1s : i64
        let s_2_23: i64 = 1;
        // C s_2_24: cast zx s_2_23 -> i
        let s_2_24: i128 = (i128::try_from(s_2_23).unwrap());
        // C s_2_25: const #61s : i
        let s_2_25: i128 = 61;
        // C s_2_26: add s_2_25 s_2_24
        let s_2_26: i128 = (s_2_25 + s_2_24);
        // D s_2_27: bit-extract s_2_22 s_2_20 s_2_26
        let s_2_27: Bits = (Bits::new(
            ((s_2_22) >> (s_2_20)).value(),
            u16::try_from(s_2_26).unwrap(),
        ));
        // D s_2_28: cast reint s_2_27 -> u62
        let s_2_28: u64 = (s_2_27.value() as u64);
        // D s_2_29: cast zx s_2_19 -> bv
        let s_2_29: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_30: cast zx s_2_28 -> bv
        let s_2_30: Bits = Bits::new(s_2_28 as u128, 62u16);
        // D s_2_31: cast reint s_2_29 -> u128
        let s_2_31: u128 = (s_2_29.value() as u128);
        // D s_2_32: size-of s_2_29
        let s_2_32: u16 = s_2_29.length();
        // D s_2_33: cast reint s_2_30 -> u128
        let s_2_33: u128 = (s_2_30.value() as u128);
        // D s_2_34: size-of s_2_30
        let s_2_34: u16 = s_2_30.length();
        // D s_2_35: lsl s_2_31 s_2_34
        let s_2_35: u128 = s_2_31 << s_2_34;
        // D s_2_36: or s_2_35 s_2_33
        let s_2_36: u128 = ((s_2_35) | (s_2_33));
        // D s_2_37: add s_2_32 s_2_34
        let s_2_37: u16 = (s_2_32 + s_2_34);
        // D s_2_38: create-bits s_2_36 s_2_37
        let s_2_38: Bits = Bits::new(s_2_36, s_2_37);
        // D s_2_39: cast reint s_2_38 -> u63
        let s_2_39: u64 = (s_2_38.value() as u64);
        // C s_2_40: const #63s : i
        let s_2_40: i128 = 63;
        // D s_2_41: read-var key0:u64
        let s_2_41: u64 = fn_state.key0;
        // D s_2_42: cast zx s_2_41 -> bv
        let s_2_42: Bits = Bits::new(s_2_41 as u128, 64u16);
        // C s_2_43: const #1u : u64
        let s_2_43: u64 = 1;
        // D s_2_44: bit-extract s_2_42 s_2_40 s_2_43
        let s_2_44: Bits = (Bits::new(
            ((s_2_42) >> (s_2_40)).value(),
            u16::try_from(s_2_43).unwrap(),
        ));
        // D s_2_45: cast reint s_2_44 -> u8
        let s_2_45: bool = ((s_2_44.value()) != 0);
        // C s_2_46: const #0s : i
        let s_2_46: i128 = 0;
        // C s_2_47: const #0u : u64
        let s_2_47: u64 = 0;
        // D s_2_48: cast zx s_2_45 -> u64
        let s_2_48: u64 = (s_2_45 as u64);
        // C s_2_49: const #1u : u64
        let s_2_49: u64 = 1;
        // D s_2_50: and s_2_48 s_2_49
        let s_2_50: u64 = ((s_2_48) & (s_2_49));
        // D s_2_51: cmp-eq s_2_50 s_2_49
        let s_2_51: bool = ((s_2_50) == (s_2_49));
        // D s_2_52: lsl s_2_48 s_2_46
        let s_2_52: u64 = s_2_48 << s_2_46;
        // D s_2_53: or s_2_47 s_2_52
        let s_2_53: u64 = ((s_2_47) | (s_2_52));
        // D s_2_54: cmpl s_2_52
        let s_2_54: u64 = !s_2_52;
        // D s_2_55: and s_2_47 s_2_54
        let s_2_55: u64 = ((s_2_47) & (s_2_54));
        // D s_2_56: select s_2_51 s_2_53 s_2_55
        let s_2_56: u64 = if s_2_51 { s_2_53 } else { s_2_55 };
        // D s_2_57: cast trunc s_2_56 -> u8
        let s_2_57: bool = ((s_2_56) != 0);
        // C s_2_58: const #1s : i
        let s_2_58: i128 = 1;
        // D s_2_59: read-var key0:u64
        let s_2_59: u64 = fn_state.key0;
        // D s_2_60: cast zx s_2_59 -> bv
        let s_2_60: Bits = Bits::new(s_2_59 as u128, 64u16);
        // C s_2_61: const #1u : u64
        let s_2_61: u64 = 1;
        // D s_2_62: bit-extract s_2_60 s_2_58 s_2_61
        let s_2_62: Bits = (Bits::new(
            ((s_2_60) >> (s_2_58)).value(),
            u16::try_from(s_2_61).unwrap(),
        ));
        // D s_2_63: cast reint s_2_62 -> u8
        let s_2_63: bool = ((s_2_62.value()) != 0);
        // C s_2_64: const #0s : i
        let s_2_64: i128 = 0;
        // C s_2_65: const #0u : u64
        let s_2_65: u64 = 0;
        // D s_2_66: cast zx s_2_63 -> u64
        let s_2_66: u64 = (s_2_63 as u64);
        // C s_2_67: const #1u : u64
        let s_2_67: u64 = 1;
        // D s_2_68: and s_2_66 s_2_67
        let s_2_68: u64 = ((s_2_66) & (s_2_67));
        // D s_2_69: cmp-eq s_2_68 s_2_67
        let s_2_69: bool = ((s_2_68) == (s_2_67));
        // D s_2_70: lsl s_2_66 s_2_64
        let s_2_70: u64 = s_2_66 << s_2_64;
        // D s_2_71: or s_2_65 s_2_70
        let s_2_71: u64 = ((s_2_65) | (s_2_70));
        // D s_2_72: cmpl s_2_70
        let s_2_72: u64 = !s_2_70;
        // D s_2_73: and s_2_65 s_2_72
        let s_2_73: u64 = ((s_2_65) & (s_2_72));
        // D s_2_74: select s_2_69 s_2_71 s_2_73
        let s_2_74: u64 = if s_2_69 { s_2_71 } else { s_2_73 };
        // D s_2_75: cast trunc s_2_74 -> u8
        let s_2_75: bool = ((s_2_74) != 0);
        // D s_2_76: cast zx s_2_57 -> bv
        let s_2_76: Bits = Bits::new(s_2_57 as u128, 1u16);
        // D s_2_77: cast zx s_2_75 -> bv
        let s_2_77: Bits = Bits::new(s_2_75 as u128, 1u16);
        // D s_2_78: xor s_2_76 s_2_77
        let s_2_78: Bits = ((s_2_76) ^ (s_2_77));
        // D s_2_79: cast reint s_2_78 -> u8
        let s_2_79: bool = ((s_2_78.value()) != 0);
        // D s_2_80: cast zx s_2_39 -> bv
        let s_2_80: Bits = Bits::new(s_2_39 as u128, 63u16);
        // D s_2_81: cast zx s_2_79 -> bv
        let s_2_81: Bits = Bits::new(s_2_79 as u128, 1u16);
        // D s_2_82: cast reint s_2_80 -> u128
        let s_2_82: u128 = (s_2_80.value() as u128);
        // D s_2_83: size-of s_2_80
        let s_2_83: u16 = s_2_80.length();
        // D s_2_84: cast reint s_2_81 -> u128
        let s_2_84: u128 = (s_2_81.value() as u128);
        // D s_2_85: size-of s_2_81
        let s_2_85: u16 = s_2_81.length();
        // D s_2_86: lsl s_2_82 s_2_85
        let s_2_86: u128 = s_2_82 << s_2_85;
        // D s_2_87: or s_2_86 s_2_84
        let s_2_87: u128 = ((s_2_86) | (s_2_84));
        // D s_2_88: add s_2_83 s_2_85
        let s_2_88: u16 = (s_2_83 + s_2_85);
        // D s_2_89: create-bits s_2_87 s_2_88
        let s_2_89: Bits = Bits::new(s_2_87, s_2_88);
        // D s_2_90: cast reint s_2_89 -> u64
        let s_2_90: u64 = (s_2_89.value() as u64);
        // D s_2_91: write-var modk0 <= s_2_90
        fn_state.modk0 = s_2_90;
        // D s_2_92: read-var modifier:u64
        let s_2_92: u64 = fn_state.modifier;
        // D s_2_93: write-var runningmod <= s_2_92
        fn_state.runningmod = s_2_92;
        // D s_2_94: read-var data:u64
        let s_2_94: u64 = fn_state.data;
        // D s_2_95: cast zx s_2_94 -> bv
        let s_2_95: Bits = Bits::new(s_2_94 as u128, 64u16);
        // D s_2_96: read-var key0:u64
        let s_2_96: u64 = fn_state.key0;
        // D s_2_97: cast zx s_2_96 -> bv
        let s_2_97: Bits = Bits::new(s_2_96 as u128, 64u16);
        // D s_2_98: xor s_2_95 s_2_97
        let s_2_98: Bits = ((s_2_95) ^ (s_2_97));
        // D s_2_99: cast reint s_2_98 -> u64
        let s_2_99: u64 = (s_2_98.value() as u64);
        // D s_2_100: write-var workingval <= s_2_99
        fn_state.workingval = s_2_99;
        // C s_2_101: const #0s : i64
        let s_2_101: i64 = 0;
        // D s_2_102: read-var iterationsshadow#257:i64
        let s_2_102: i64 = fn_state.iterationsshadow_257;
        // D s_2_103: write-var gs#14641 <= s_2_102
        fn_state.gs_14641 = s_2_102;
        // D s_2_104: write-var i <= s_2_101
        fn_state.i = s_2_101;
        // N s_2_105: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var i:i64
        let s_3_0: i64 = fn_state.i;
        // D s_3_1: read-var gs#14641:i64
        let s_3_1: i64 = fn_state.gs_14641;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b11 b4
        if s_3_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var key1:u64
        let s_4_0: u64 = fn_state.key1;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 64u16);
        // D s_4_2: read-var runningmod:u64
        let s_4_2: u64 = fn_state.runningmod;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_4: xor s_4_1 s_4_3
        let s_4_4: Bits = ((s_4_1) ^ (s_4_3));
        // D s_4_5: cast reint s_4_4 -> u64
        let s_4_5: u64 = (s_4_4.value() as u64);
        // D s_4_6: write-var roundkey <= s_4_5
        fn_state.roundkey = s_4_5;
        // D s_4_7: read-var workingval:u64
        let s_4_7: u64 = fn_state.workingval;
        // D s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 64u16);
        // D s_4_9: read-var roundkey:u64
        let s_4_9: u64 = fn_state.roundkey;
        // D s_4_10: cast zx s_4_9 -> bv
        let s_4_10: Bits = Bits::new(s_4_9 as u128, 64u16);
        // D s_4_11: xor s_4_8 s_4_10
        let s_4_11: Bits = ((s_4_8) ^ (s_4_10));
        // D s_4_12: cast reint s_4_11 -> u64
        let s_4_12: u64 = (s_4_11.value() as u64);
        // D s_4_13: write-var workingval <= s_4_12
        fn_state.workingval = s_4_12;
        // C s_4_14: const #20560u : u32
        let s_4_14: u32 = 20560;
        // D s_4_15: read-reg s_4_14:[u64; 5]
        let s_4_15: [u64; 5usize] = {
            let value = state.read_register::<[u64; 5usize]>(s_4_14 as isize);
            tracer.read_register(s_4_14 as isize, value);
            value
        };
        // D s_4_16: read-var i:i64
        let s_4_16: i64 = fn_state.i;
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: read-element s_4_15[s_4_17]
        let s_4_18: u64 = s_4_15[(s_4_17) as usize];
        // D s_4_19: read-var workingval:u64
        let s_4_19: u64 = fn_state.workingval;
        // D s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 64u16);
        // D s_4_21: cast zx s_4_18 -> bv
        let s_4_21: Bits = Bits::new(s_4_18 as u128, 64u16);
        // D s_4_22: xor s_4_20 s_4_21
        let s_4_22: Bits = ((s_4_20) ^ (s_4_21));
        // D s_4_23: cast reint s_4_22 -> u64
        let s_4_23: u64 = (s_4_22.value() as u64);
        // D s_4_24: write-var workingval <= s_4_23
        fn_state.workingval = s_4_23;
        // C s_4_25: const #0s : i
        let s_4_25: i128 = 0;
        // D s_4_26: read-var i:i64
        let s_4_26: i64 = fn_state.i;
        // D s_4_27: cast zx s_4_26 -> i
        let s_4_27: i128 = (i128::try_from(s_4_26).unwrap());
        // D s_4_28: cmp-gt s_4_27 s_4_25
        let s_4_28: bool = ((s_4_27) > (s_4_25));
        // N s_4_29: branch s_4_28 b10 b5
        if s_4_28 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_6_0: read-var isqarma3:u8
        let s_6_0: bool = fn_state.isqarma3;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var workingval:u64
        let s_7_0: u64 = fn_state.workingval;
        // D s_7_1: call PACSub(s_7_0)
        let s_7_1: u64 = PACSub(state, tracer, s_7_0);
        // D s_7_2: write-var workingval <= s_7_1
        fn_state.workingval = s_7_1;
        // N s_7_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var runningmod:u64
        let s_8_1: u64 = fn_state.runningmod;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 64u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #63s : i
        let s_8_5: i128 = 63;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_0 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u64
        let s_8_8: u64 = (s_8_7.value() as u64);
        // D s_8_9: call TweakShuffle(s_8_8)
        let s_8_9: u64 = TweakShuffle(state, tracer, s_8_8);
        // D s_8_10: write-var runningmod <= s_8_9
        fn_state.runningmod = s_8_9;
        // D s_8_11: read-var i:i64
        let s_8_11: i64 = fn_state.i;
        // C s_8_12: const #1s : i64
        let s_8_12: i64 = 1;
        // D s_8_13: add s_8_11 s_8_12
        let s_8_13: i64 = (s_8_11 + s_8_12);
        // D s_8_14: write-var i <= s_8_13
        fn_state.i = s_8_13;
        // N s_8_15: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_9_0: read-var workingval:u64
        let s_9_0: u64 = fn_state.workingval;
        // D s_9_1: call PACSub1(s_9_0)
        let s_9_1: u64 = PACSub1(state, tracer, s_9_0);
        // D s_9_2: write-var workingval <= s_9_1
        fn_state.workingval = s_9_1;
        // N s_9_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_10_0: read-var workingval:u64
        let s_10_0: u64 = fn_state.workingval;
        // D s_10_1: call PACCellShuffle(s_10_0)
        let s_10_1: u64 = PACCellShuffle(state, tracer, s_10_0);
        // D s_10_2: write-var workingval <= s_10_1
        fn_state.workingval = s_10_1;
        // D s_10_3: read-var workingval:u64
        let s_10_3: u64 = fn_state.workingval;
        // D s_10_4: call PACMult(s_10_3)
        let s_10_4: u64 = PACMult(state, tracer, s_10_3);
        // D s_10_5: write-var workingval <= s_10_4
        fn_state.workingval = s_10_4;
        // N s_10_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_11_0: read-var modk0:u64
        let s_11_0: u64 = fn_state.modk0;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 64u16);
        // D s_11_2: read-var runningmod:u64
        let s_11_2: u64 = fn_state.runningmod;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 64u16);
        // D s_11_4: xor s_11_1 s_11_3
        let s_11_4: Bits = ((s_11_1) ^ (s_11_3));
        // D s_11_5: cast reint s_11_4 -> u64
        let s_11_5: u64 = (s_11_4.value() as u64);
        // D s_11_6: write-var roundkey <= s_11_5
        fn_state.roundkey = s_11_5;
        // D s_11_7: read-var workingval:u64
        let s_11_7: u64 = fn_state.workingval;
        // D s_11_8: cast zx s_11_7 -> bv
        let s_11_8: Bits = Bits::new(s_11_7 as u128, 64u16);
        // D s_11_9: read-var roundkey:u64
        let s_11_9: u64 = fn_state.roundkey;
        // D s_11_10: cast zx s_11_9 -> bv
        let s_11_10: Bits = Bits::new(s_11_9 as u128, 64u16);
        // D s_11_11: xor s_11_8 s_11_10
        let s_11_11: Bits = ((s_11_8) ^ (s_11_10));
        // D s_11_12: cast reint s_11_11 -> u64
        let s_11_12: u64 = (s_11_11.value() as u64);
        // D s_11_13: write-var workingval <= s_11_12
        fn_state.workingval = s_11_12;
        // D s_11_14: read-var workingval:u64
        let s_11_14: u64 = fn_state.workingval;
        // D s_11_15: call PACCellShuffle(s_11_14)
        let s_11_15: u64 = PACCellShuffle(state, tracer, s_11_14);
        // D s_11_16: write-var workingval <= s_11_15
        fn_state.workingval = s_11_15;
        // D s_11_17: read-var workingval:u64
        let s_11_17: u64 = fn_state.workingval;
        // D s_11_18: call PACMult(s_11_17)
        let s_11_18: u64 = PACMult(state, tracer, s_11_17);
        // D s_11_19: write-var workingval <= s_11_18
        fn_state.workingval = s_11_18;
        // D s_11_20: read-var isqarma3:u8
        let s_11_20: bool = fn_state.isqarma3;
        // N s_11_21: branch s_11_20 b26 b12
        if s_11_20 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_12_0: read-var workingval:u64
        let s_12_0: u64 = fn_state.workingval;
        // D s_12_1: call PACSub(s_12_0)
        let s_12_1: u64 = PACSub(state, tracer, s_12_0);
        // D s_12_2: write-var workingval <= s_12_1
        fn_state.workingval = s_12_1;
        // N s_12_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_13_0: read-var workingval:u64
        let s_13_0: u64 = fn_state.workingval;
        // D s_13_1: call PACCellShuffle(s_13_0)
        let s_13_1: u64 = PACCellShuffle(state, tracer, s_13_0);
        // D s_13_2: write-var workingval <= s_13_1
        fn_state.workingval = s_13_1;
        // D s_13_3: read-var workingval:u64
        let s_13_3: u64 = fn_state.workingval;
        // D s_13_4: call PACMult(s_13_3)
        let s_13_4: u64 = PACMult(state, tracer, s_13_3);
        // D s_13_5: write-var workingval <= s_13_4
        fn_state.workingval = s_13_4;
        // D s_13_6: read-var key1:u64
        let s_13_6: u64 = fn_state.key1;
        // D s_13_7: cast zx s_13_6 -> bv
        let s_13_7: Bits = Bits::new(s_13_6 as u128, 64u16);
        // D s_13_8: read-var workingval:u64
        let s_13_8: u64 = fn_state.workingval;
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 64u16);
        // D s_13_10: xor s_13_7 s_13_9
        let s_13_10: Bits = ((s_13_7) ^ (s_13_9));
        // D s_13_11: cast reint s_13_10 -> u64
        let s_13_11: u64 = (s_13_10.value() as u64);
        // D s_13_12: write-var workingval <= s_13_11
        fn_state.workingval = s_13_11;
        // D s_13_13: read-var workingval:u64
        let s_13_13: u64 = fn_state.workingval;
        // D s_13_14: call PACCellInvShuffle(s_13_13)
        let s_13_14: u64 = PACCellInvShuffle(state, tracer, s_13_13);
        // D s_13_15: write-var workingval <= s_13_14
        fn_state.workingval = s_13_14;
        // D s_13_16: read-var isqarma3:u8
        let s_13_16: bool = fn_state.isqarma3;
        // N s_13_17: branch s_13_16 b25 b14
        if s_13_16 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_14_0: read-var workingval:u64
        let s_14_0: u64 = fn_state.workingval;
        // D s_14_1: call PACInvSub(s_14_0)
        let s_14_1: u64 = PACInvSub(state, tracer, s_14_0);
        // D s_14_2: write-var workingval <= s_14_1
        fn_state.workingval = s_14_1;
        // N s_14_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_15_0: read-var workingval:u64
        let s_15_0: u64 = fn_state.workingval;
        // D s_15_1: call PACMult(s_15_0)
        let s_15_1: u64 = PACMult(state, tracer, s_15_0);
        // D s_15_2: write-var workingval <= s_15_1
        fn_state.workingval = s_15_1;
        // D s_15_3: read-var workingval:u64
        let s_15_3: u64 = fn_state.workingval;
        // D s_15_4: call PACCellInvShuffle(s_15_3)
        let s_15_4: u64 = PACCellInvShuffle(state, tracer, s_15_3);
        // D s_15_5: write-var workingval <= s_15_4
        fn_state.workingval = s_15_4;
        // D s_15_6: read-var workingval:u64
        let s_15_6: u64 = fn_state.workingval;
        // D s_15_7: cast zx s_15_6 -> bv
        let s_15_7: Bits = Bits::new(s_15_6 as u128, 64u16);
        // D s_15_8: read-var key0:u64
        let s_15_8: u64 = fn_state.key0;
        // D s_15_9: cast zx s_15_8 -> bv
        let s_15_9: Bits = Bits::new(s_15_8 as u128, 64u16);
        // D s_15_10: xor s_15_7 s_15_9
        let s_15_10: Bits = ((s_15_7) ^ (s_15_9));
        // D s_15_11: cast reint s_15_10 -> u64
        let s_15_11: u64 = (s_15_10.value() as u64);
        // D s_15_12: write-var workingval <= s_15_11
        fn_state.workingval = s_15_11;
        // D s_15_13: read-var workingval:u64
        let s_15_13: u64 = fn_state.workingval;
        // D s_15_14: cast zx s_15_13 -> bv
        let s_15_14: Bits = Bits::new(s_15_13 as u128, 64u16);
        // D s_15_15: read-var runningmod:u64
        let s_15_15: u64 = fn_state.runningmod;
        // D s_15_16: cast zx s_15_15 -> bv
        let s_15_16: Bits = Bits::new(s_15_15 as u128, 64u16);
        // D s_15_17: xor s_15_14 s_15_16
        let s_15_17: Bits = ((s_15_14) ^ (s_15_16));
        // D s_15_18: cast reint s_15_17 -> u64
        let s_15_18: u64 = (s_15_17.value() as u64);
        // D s_15_19: write-var workingval <= s_15_18
        fn_state.workingval = s_15_18;
        // C s_15_20: const #0s : i64
        let s_15_20: i64 = 0;
        // D s_15_21: read-var iterationsshadow#257:i64
        let s_15_21: i64 = fn_state.iterationsshadow_257;
        // D s_15_22: write-var gs#14656 <= s_15_21
        fn_state.gs_14656 = s_15_21;
        // D s_15_23: write-var u#385 <= s_15_20
        fn_state.u_385 = s_15_20;
        // N s_15_24: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_16_0: read-var u#385:i64
        let s_16_0: i64 = fn_state.u_385;
        // D s_16_1: read-var gs#14656:i64
        let s_16_1: i64 = fn_state.gs_14656;
        // D s_16_2: cmp-gt s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) > (s_16_1));
        // N s_16_3: branch s_16_2 b24 b17
        if s_16_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_17_0: read-var isqarma3:u8
        let s_17_0: bool = fn_state.isqarma3;
        // N s_17_1: branch s_17_0 b23 b18
        if s_17_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_18_0: read-var workingval:u64
        let s_18_0: u64 = fn_state.workingval;
        // D s_18_1: call PACInvSub(s_18_0)
        let s_18_1: u64 = PACInvSub(state, tracer, s_18_0);
        // D s_18_2: write-var workingval <= s_18_1
        fn_state.workingval = s_18_1;
        // N s_18_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_19_0: read-var u#385:i64
        let s_19_0: i64 = fn_state.u_385;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var iterationsshadow#257:i64
        let s_19_2: i64 = fn_state.iterationsshadow_257;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: cmp-lt s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) < (s_19_3));
        // N s_19_5: branch s_19_4 b22 b20
        if s_19_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_21_0: const #0s : i
        let s_21_0: i128 = 0;
        // D s_21_1: read-var runningmod:u64
        let s_21_1: u64 = fn_state.runningmod;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 64u16);
        // C s_21_3: const #1s : i64
        let s_21_3: i64 = 1;
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #63s : i
        let s_21_5: i128 = 63;
        // C s_21_6: add s_21_5 s_21_4
        let s_21_6: i128 = (s_21_5 + s_21_4);
        // D s_21_7: bit-extract s_21_2 s_21_0 s_21_6
        let s_21_7: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_6).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u64
        let s_21_8: u64 = (s_21_7.value() as u64);
        // D s_21_9: call TweakInvShuffle(s_21_8)
        let s_21_9: u64 = TweakInvShuffle(state, tracer, s_21_8);
        // D s_21_10: write-var runningmod <= s_21_9
        fn_state.runningmod = s_21_9;
        // D s_21_11: read-var key1:u64
        let s_21_11: u64 = fn_state.key1;
        // D s_21_12: cast zx s_21_11 -> bv
        let s_21_12: Bits = Bits::new(s_21_11 as u128, 64u16);
        // D s_21_13: read-var runningmod:u64
        let s_21_13: u64 = fn_state.runningmod;
        // D s_21_14: cast zx s_21_13 -> bv
        let s_21_14: Bits = Bits::new(s_21_13 as u128, 64u16);
        // D s_21_15: xor s_21_12 s_21_14
        let s_21_15: Bits = ((s_21_12) ^ (s_21_14));
        // D s_21_16: cast reint s_21_15 -> u64
        let s_21_16: u64 = (s_21_15.value() as u64);
        // D s_21_17: write-var roundkey <= s_21_16
        fn_state.roundkey = s_21_16;
        // D s_21_18: read-var iterationsshadow#257:i64
        let s_21_18: i64 = fn_state.iterationsshadow_257;
        // D s_21_19: cast zx s_21_18 -> i
        let s_21_19: i128 = (i128::try_from(s_21_18).unwrap());
        // D s_21_20: read-var u#385:i64
        let s_21_20: i64 = fn_state.u_385;
        // D s_21_21: cast zx s_21_20 -> i
        let s_21_21: i128 = (i128::try_from(s_21_20).unwrap());
        // D s_21_22: sub s_21_19 s_21_21
        let s_21_22: i128 = ((s_21_19) - (s_21_21));
        // D s_21_23: cast reint s_21_22 -> i64
        let s_21_23: i64 = (s_21_22 as i64);
        // C s_21_24: const #20560u : u32
        let s_21_24: u32 = 20560;
        // D s_21_25: read-reg s_21_24:[u64; 5]
        let s_21_25: [u64; 5usize] = {
            let value = state.read_register::<[u64; 5usize]>(s_21_24 as isize);
            tracer.read_register(s_21_24 as isize, value);
            value
        };
        // D s_21_26: cast zx s_21_23 -> i
        let s_21_26: i128 = (i128::try_from(s_21_23).unwrap());
        // D s_21_27: read-element s_21_25[s_21_26]
        let s_21_27: u64 = s_21_25[(s_21_26) as usize];
        // D s_21_28: read-var workingval:u64
        let s_21_28: u64 = fn_state.workingval;
        // D s_21_29: cast zx s_21_28 -> bv
        let s_21_29: Bits = Bits::new(s_21_28 as u128, 64u16);
        // D s_21_30: cast zx s_21_27 -> bv
        let s_21_30: Bits = Bits::new(s_21_27 as u128, 64u16);
        // D s_21_31: xor s_21_29 s_21_30
        let s_21_31: Bits = ((s_21_29) ^ (s_21_30));
        // D s_21_32: cast reint s_21_31 -> u64
        let s_21_32: u64 = (s_21_31.value() as u64);
        // D s_21_33: write-var workingval <= s_21_32
        fn_state.workingval = s_21_32;
        // D s_21_34: read-var workingval:u64
        let s_21_34: u64 = fn_state.workingval;
        // D s_21_35: cast zx s_21_34 -> bv
        let s_21_35: Bits = Bits::new(s_21_34 as u128, 64u16);
        // D s_21_36: read-var roundkey:u64
        let s_21_36: u64 = fn_state.roundkey;
        // D s_21_37: cast zx s_21_36 -> bv
        let s_21_37: Bits = Bits::new(s_21_36 as u128, 64u16);
        // D s_21_38: xor s_21_35 s_21_37
        let s_21_38: Bits = ((s_21_35) ^ (s_21_37));
        // D s_21_39: cast reint s_21_38 -> u64
        let s_21_39: u64 = (s_21_38.value() as u64);
        // D s_21_40: write-var workingval <= s_21_39
        fn_state.workingval = s_21_39;
        // D s_21_41: read-var workingval:u64
        let s_21_41: u64 = fn_state.workingval;
        // D s_21_42: cast zx s_21_41 -> bv
        let s_21_42: Bits = Bits::new(s_21_41 as u128, 64u16);
        // C s_21_43: const #13883517620612518109u : u64
        let s_21_43: u64 = 13883517620612518109;
        // C s_21_44: cast zx s_21_43 -> bv
        let s_21_44: Bits = Bits::new(s_21_43 as u128, 64u16);
        // D s_21_45: xor s_21_42 s_21_44
        let s_21_45: Bits = ((s_21_42) ^ (s_21_44));
        // D s_21_46: cast reint s_21_45 -> u64
        let s_21_46: u64 = (s_21_45.value() as u64);
        // D s_21_47: write-var workingval <= s_21_46
        fn_state.workingval = s_21_46;
        // D s_21_48: read-var u#385:i64
        let s_21_48: i64 = fn_state.u_385;
        // C s_21_49: const #1s : i64
        let s_21_49: i64 = 1;
        // D s_21_50: add s_21_48 s_21_49
        let s_21_50: i64 = (s_21_48 + s_21_49);
        // D s_21_51: write-var u#385 <= s_21_50
        fn_state.u_385 = s_21_50;
        // N s_21_52: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_22_0: read-var workingval:u64
        let s_22_0: u64 = fn_state.workingval;
        // D s_22_1: call PACMult(s_22_0)
        let s_22_1: u64 = PACMult(state, tracer, s_22_0);
        // D s_22_2: write-var workingval <= s_22_1
        fn_state.workingval = s_22_1;
        // D s_22_3: read-var workingval:u64
        let s_22_3: u64 = fn_state.workingval;
        // D s_22_4: call PACCellInvShuffle(s_22_3)
        let s_22_4: u64 = PACCellInvShuffle(state, tracer, s_22_3);
        // D s_22_5: write-var workingval <= s_22_4
        fn_state.workingval = s_22_4;
        // N s_22_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_23_0: read-var workingval:u64
        let s_23_0: u64 = fn_state.workingval;
        // D s_23_1: call PACSub1(s_23_0)
        let s_23_1: u64 = PACSub1(state, tracer, s_23_0);
        // D s_23_2: write-var workingval <= s_23_1
        fn_state.workingval = s_23_1;
        // N s_23_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_24_0: read-var workingval:u64
        let s_24_0: u64 = fn_state.workingval;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 64u16);
        // D s_24_2: read-var modk0:u64
        let s_24_2: u64 = fn_state.modk0;
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 64u16);
        // D s_24_4: xor s_24_1 s_24_3
        let s_24_4: Bits = ((s_24_1) ^ (s_24_3));
        // D s_24_5: cast reint s_24_4 -> u64
        let s_24_5: u64 = (s_24_4.value() as u64);
        // N s_24_6: return s_24_5
        return s_24_5;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_25_0: read-var workingval:u64
        let s_25_0: u64 = fn_state.workingval;
        // D s_25_1: call PACSub1(s_25_0)
        let s_25_1: u64 = PACSub1(state, tracer, s_25_0);
        // D s_25_2: write-var workingval <= s_25_1
        fn_state.workingval = s_25_1;
        // N s_25_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_26_0: read-var workingval:u64
        let s_26_0: u64 = fn_state.workingval;
        // D s_26_1: call PACSub1(s_26_0)
        let s_26_1: u64 = PACSub1(state, tracer, s_26_0);
        // D s_26_2: write-var workingval <= s_26_1
        fn_state.workingval = s_26_1;
        // N s_26_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_27_0: const #2s : i64
        let s_27_0: i64 = 2;
        // D s_27_1: write-var iterations <= s_27_0
        fn_state.iterations = s_27_0;
        // N s_27_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
