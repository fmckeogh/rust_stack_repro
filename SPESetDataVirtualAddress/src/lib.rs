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
use Zeros::*;
use EffectiveTBI::*;
use u__IMPDEF_boolean::*;
use common::*;
pub fn SPESetDataVirtualAddress<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        non_tbi_is_zeros: bool,
        gs_20062: bool,
        vaddress: u64,
    }
    let fn_state = FunctionState {
        vaddress,
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
        // D s_0_2: read-var vaddress:u64
        let s_0_2: u64 = fn_state.vaddress;
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // D s_0_4: call EffectiveTBI(s_0_2, s_0_3, s_0_1)
        let s_0_4: bool = EffectiveTBI(state, tracer, s_0_2, s_0_3, s_0_1);
        // C s_0_5: const #"SPE non-tbi tag is zero" : str
        let s_0_5: &'static str = "SPE non-tbi tag is zero";
        // S s_0_6: call __IMPDEF_boolean(s_0_5)
        let s_0_6: bool = u__IMPDEF_boolean(state, tracer, s_0_5);
        // D s_0_7: write-var non_tbi_is_zeros <= s_0_6
        fn_state.non_tbi_is_zeros = s_0_6;
        // D s_0_8: cast zx s_0_4 -> bv
        let s_0_8: Bits = Bits::new(s_0_4 as u128, 1u16);
        // C s_0_9: const #1u : u8
        let s_0_9: bool = true;
        // C s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 1u16);
        // D s_0_11: cmp-eq s_0_8 s_0_10
        let s_0_11: bool = ((s_0_8) == (s_0_10));
        // N s_0_12: branch s_0_11 b6 b1
        if s_0_11 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var non_tbi_is_zeros:u8
        let s_1_0: bool = fn_state.non_tbi_is_zeros;
        // D s_1_1: not s_1_0
        let s_1_1: bool = !s_1_0;
        // D s_1_2: write-var gs#20062 <= s_1_1
        fn_state.gs_20062 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#20062:u8
        let s_2_0: bool = fn_state.gs_20062;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #13776u : u32
        let s_3_0: u32 = 13776;
        // D s_3_1: read-reg s_3_0:[u64; 32]
        let s_3_1: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #1032u : u32
        let s_3_2: u32 = 1032;
        // D s_3_3: read-reg s_3_2:i64
        let s_3_3: i64 = {
            let value = state.read_register::<i64>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-element s_3_1[s_3_4]
        let s_3_5: u64 = s_3_1[(s_3_4) as usize];
        // C s_3_6: const #8s : i
        let s_3_6: i128 = 8;
        // S s_3_7: call Zeros(s_3_6)
        let s_3_7: Bits = Zeros(state, tracer, s_3_6);
        // S s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // C s_3_9: const #56s : i
        let s_3_9: i128 = 56;
        // D s_3_10: cast zx s_3_5 -> bv
        let s_3_10: Bits = Bits::new(s_3_5 as u128, 64u16);
        // S s_3_11: cast zx s_3_8 -> bv
        let s_3_11: Bits = Bits::new(s_3_8 as u128, 8u16);
        // C s_3_12: const #7s : i
        let s_3_12: i128 = 7;
        // C s_3_13: const #1u : u64
        let s_3_13: u64 = 1;
        // C s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 64u16);
        // C s_3_15: lsl s_3_14 s_3_12
        let s_3_15: Bits = s_3_14 << s_3_12;
        // C s_3_16: sub s_3_15 s_3_14
        let s_3_16: Bits = ((s_3_15) - (s_3_14));
        // S s_3_17: and s_3_11 s_3_16
        let s_3_17: Bits = ((s_3_11) & (s_3_16));
        // S s_3_18: lsl s_3_17 s_3_9
        let s_3_18: Bits = s_3_17 << s_3_9;
        // C s_3_19: lsl s_3_16 s_3_9
        let s_3_19: Bits = s_3_16 << s_3_9;
        // C s_3_20: cmpl s_3_19
        let s_3_20: Bits = !s_3_19;
        // D s_3_21: and s_3_10 s_3_20
        let s_3_21: Bits = ((s_3_10) & (s_3_20));
        // D s_3_22: or s_3_21 s_3_18
        let s_3_22: Bits = ((s_3_21) | (s_3_18));
        // D s_3_23: cast reint s_3_22 -> u64
        let s_3_23: u64 = (s_3_22.value() as u64);
        // C s_3_24: const #13776u : u32
        let s_3_24: u32 = 13776;
        // D s_3_25: read-reg s_3_24:[u64; 32]
        let s_3_25: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_3_24 as isize);
            tracer.read_register(s_3_24 as isize, value);
            value
        };
        // C s_3_26: const #1032u : u32
        let s_3_26: u32 = 1032;
        // D s_3_27: read-reg s_3_26:i64
        let s_3_27: i64 = {
            let value = state.read_register::<i64>(s_3_26 as isize);
            tracer.read_register(s_3_26 as isize, value);
            value
        };
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: mutate-element s_3_25[s_3_28] <= s_3_23
        let s_3_29: [u64; 32usize] = {
            let mut local = s_3_25.clone();
            local[(s_3_28) as usize] = s_3_23;
            local
        };
        // D s_3_30: cast cvt s_3_29 -> [u64; 0]
        let s_3_30: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_3_29);
        // D s_3_31: cast cvt s_3_30 -> [u64; 32]
        let s_3_31: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_3_30);
            buf
        };
        // C s_3_32: const #13776u : u32
        let s_3_32: u32 = 13776;
        // N s_3_33: write-reg s_3_32 <= s_3_31
        let s_3_33: () = {
            state.write_register::<[u64; 32usize]>(s_3_32 as isize, s_3_31);
            tracer.write_register(s_3_32 as isize, s_3_31);
        };
        // C s_3_34: const #13776u : u32
        let s_3_34: u32 = 13776;
        // D s_3_35: read-reg s_3_34:[u64; 32]
        let s_3_35: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_3_34 as isize);
            tracer.read_register(s_3_34 as isize, value);
            value
        };
        // C s_3_36: const #1032u : u32
        let s_3_36: u32 = 1032;
        // D s_3_37: read-reg s_3_36:i64
        let s_3_37: i64 = {
            let value = state.read_register::<i64>(s_3_36 as isize);
            tracer.read_register(s_3_36 as isize, value);
            value
        };
        // D s_3_38: cast zx s_3_37 -> i
        let s_3_38: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_39: read-element s_3_35[s_3_38]
        let s_3_39: u64 = s_3_35[(s_3_38) as usize];
        // C s_3_40: const #0s : i
        let s_3_40: i128 = 0;
        // D s_3_41: read-var vaddress:u64
        let s_3_41: u64 = fn_state.vaddress;
        // D s_3_42: cast zx s_3_41 -> bv
        let s_3_42: Bits = Bits::new(s_3_41 as u128, 64u16);
        // C s_3_43: const #1s : i64
        let s_3_43: i64 = 1;
        // C s_3_44: cast zx s_3_43 -> i
        let s_3_44: i128 = (i128::try_from(s_3_43).unwrap());
        // C s_3_45: const #55s : i
        let s_3_45: i128 = 55;
        // C s_3_46: add s_3_45 s_3_44
        let s_3_46: i128 = (s_3_45 + s_3_44);
        // D s_3_47: bit-extract s_3_42 s_3_40 s_3_46
        let s_3_47: Bits = (Bits::new(
            ((s_3_42) >> (s_3_40)).value(),
            u16::try_from(s_3_46).unwrap(),
        ));
        // D s_3_48: cast reint s_3_47 -> u56
        let s_3_48: u64 = (s_3_47.value() as u64);
        // C s_3_49: const #0s : i
        let s_3_49: i128 = 0;
        // D s_3_50: cast zx s_3_39 -> bv
        let s_3_50: Bits = Bits::new(s_3_39 as u128, 64u16);
        // D s_3_51: cast zx s_3_48 -> bv
        let s_3_51: Bits = Bits::new(s_3_48 as u128, 56u16);
        // C s_3_52: const #55s : i
        let s_3_52: i128 = 55;
        // C s_3_53: const #1u : u64
        let s_3_53: u64 = 1;
        // C s_3_54: cast zx s_3_53 -> bv
        let s_3_54: Bits = Bits::new(s_3_53 as u128, 64u16);
        // C s_3_55: lsl s_3_54 s_3_52
        let s_3_55: Bits = s_3_54 << s_3_52;
        // C s_3_56: sub s_3_55 s_3_54
        let s_3_56: Bits = ((s_3_55) - (s_3_54));
        // D s_3_57: and s_3_51 s_3_56
        let s_3_57: Bits = ((s_3_51) & (s_3_56));
        // D s_3_58: lsl s_3_57 s_3_49
        let s_3_58: Bits = s_3_57 << s_3_49;
        // C s_3_59: lsl s_3_56 s_3_49
        let s_3_59: Bits = s_3_56 << s_3_49;
        // C s_3_60: cmpl s_3_59
        let s_3_60: Bits = !s_3_59;
        // D s_3_61: and s_3_50 s_3_60
        let s_3_61: Bits = ((s_3_50) & (s_3_60));
        // D s_3_62: or s_3_61 s_3_58
        let s_3_62: Bits = ((s_3_61) | (s_3_58));
        // D s_3_63: cast reint s_3_62 -> u64
        let s_3_63: u64 = (s_3_62.value() as u64);
        // C s_3_64: const #13776u : u32
        let s_3_64: u32 = 13776;
        // D s_3_65: read-reg s_3_64:[u64; 32]
        let s_3_65: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_3_64 as isize);
            tracer.read_register(s_3_64 as isize, value);
            value
        };
        // C s_3_66: const #1032u : u32
        let s_3_66: u32 = 1032;
        // D s_3_67: read-reg s_3_66:i64
        let s_3_67: i64 = {
            let value = state.read_register::<i64>(s_3_66 as isize);
            tracer.read_register(s_3_66 as isize, value);
            value
        };
        // D s_3_68: cast zx s_3_67 -> i
        let s_3_68: i128 = (i128::try_from(s_3_67).unwrap());
        // D s_3_69: mutate-element s_3_65[s_3_68] <= s_3_63
        let s_3_69: [u64; 32usize] = {
            let mut local = s_3_65.clone();
            local[(s_3_68) as usize] = s_3_63;
            local
        };
        // D s_3_70: cast cvt s_3_69 -> [u64; 0]
        let s_3_70: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_3_69);
        // D s_3_71: cast cvt s_3_70 -> [u64; 32]
        let s_3_71: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_3_70);
            buf
        };
        // C s_3_72: const #13776u : u32
        let s_3_72: u32 = 13776;
        // N s_3_73: write-reg s_3_72 <= s_3_71
        let s_3_73: () = {
            state.write_register::<[u64; 32usize]>(s_3_72 as isize, s_3_71);
            tracer.write_register(s_3_72 as isize, s_3_71);
        };
        // N s_3_74: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #11136u : u32
        let s_4_0: u32 = 11136;
        // D s_4_1: read-reg s_4_0:[u8; 32]
        let s_4_1: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // C s_4_2: const #1032u : u32
        let s_4_2: u32 = 1032;
        // D s_4_3: read-reg s_4_2:i64
        let s_4_3: i64 = {
            let value = state.read_register::<i64>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #1u : u8
        let s_4_5: bool = true;
        // D s_4_6: mutate-element s_4_1[s_4_4] <= s_4_5
        let s_4_6: [bool; 32usize] = {
            let mut local = s_4_1.clone();
            local[(s_4_4) as usize] = s_4_5;
            local
        };
        // D s_4_7: cast cvt s_4_6 -> [u8; 0]
        let s_4_7: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_4_6);
        // D s_4_8: cast cvt s_4_7 -> [u8; 32]
        let s_4_8: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_4_7);
            buf
        };
        // C s_4_9: const #11136u : u32
        let s_4_9: u32 = 11136;
        // N s_4_10: write-reg s_4_9 <= s_4_8
        let s_4_10: () = {
            state.write_register::<[bool; 32usize]>(s_4_9 as isize, s_4_8);
            tracer.write_register(s_4_9 as isize, s_4_8);
        };
        // N s_4_11: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #13776u : u32
        let s_5_0: u32 = 13776;
        // D s_5_1: read-reg s_5_0:[u64; 32]
        let s_5_1: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #1032u : u32
        let s_5_2: u32 = 1032;
        // D s_5_3: read-reg s_5_2:i64
        let s_5_3: i64 = {
            let value = state.read_register::<i64>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-element s_5_1[s_5_4]
        let s_5_5: u64 = s_5_1[(s_5_4) as usize];
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // D s_5_7: read-var vaddress:u64
        let s_5_7: u64 = fn_state.vaddress;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 64u16);
        // C s_5_9: const #1s : i64
        let s_5_9: i64 = 1;
        // C s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // C s_5_11: const #63s : i
        let s_5_11: i128 = 63;
        // C s_5_12: add s_5_11 s_5_10
        let s_5_12: i128 = (s_5_11 + s_5_10);
        // D s_5_13: bit-extract s_5_8 s_5_6 s_5_12
        let s_5_13: Bits = (Bits::new(
            ((s_5_8) >> (s_5_6)).value(),
            u16::try_from(s_5_12).unwrap(),
        ));
        // D s_5_14: cast reint s_5_13 -> u64
        let s_5_14: u64 = (s_5_13.value() as u64);
        // C s_5_15: const #0s : i
        let s_5_15: i128 = 0;
        // D s_5_16: cast zx s_5_5 -> bv
        let s_5_16: Bits = Bits::new(s_5_5 as u128, 64u16);
        // D s_5_17: cast zx s_5_14 -> bv
        let s_5_17: Bits = Bits::new(s_5_14 as u128, 64u16);
        // C s_5_18: const #63s : i
        let s_5_18: i128 = 63;
        // C s_5_19: const #1u : u64
        let s_5_19: u64 = 1;
        // C s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 64u16);
        // C s_5_21: lsl s_5_20 s_5_18
        let s_5_21: Bits = s_5_20 << s_5_18;
        // C s_5_22: sub s_5_21 s_5_20
        let s_5_22: Bits = ((s_5_21) - (s_5_20));
        // D s_5_23: and s_5_17 s_5_22
        let s_5_23: Bits = ((s_5_17) & (s_5_22));
        // D s_5_24: lsl s_5_23 s_5_15
        let s_5_24: Bits = s_5_23 << s_5_15;
        // C s_5_25: lsl s_5_22 s_5_15
        let s_5_25: Bits = s_5_22 << s_5_15;
        // C s_5_26: cmpl s_5_25
        let s_5_26: Bits = !s_5_25;
        // D s_5_27: and s_5_16 s_5_26
        let s_5_27: Bits = ((s_5_16) & (s_5_26));
        // D s_5_28: or s_5_27 s_5_24
        let s_5_28: Bits = ((s_5_27) | (s_5_24));
        // D s_5_29: cast reint s_5_28 -> u64
        let s_5_29: u64 = (s_5_28.value() as u64);
        // C s_5_30: const #13776u : u32
        let s_5_30: u32 = 13776;
        // D s_5_31: read-reg s_5_30:[u64; 32]
        let s_5_31: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_5_30 as isize);
            tracer.read_register(s_5_30 as isize, value);
            value
        };
        // C s_5_32: const #1032u : u32
        let s_5_32: u32 = 1032;
        // D s_5_33: read-reg s_5_32:i64
        let s_5_33: i64 = {
            let value = state.read_register::<i64>(s_5_32 as isize);
            tracer.read_register(s_5_32 as isize, value);
            value
        };
        // D s_5_34: cast zx s_5_33 -> i
        let s_5_34: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_35: mutate-element s_5_31[s_5_34] <= s_5_29
        let s_5_35: [u64; 32usize] = {
            let mut local = s_5_31.clone();
            local[(s_5_34) as usize] = s_5_29;
            local
        };
        // D s_5_36: cast cvt s_5_35 -> [u64; 0]
        let s_5_36: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_5_35);
        // D s_5_37: cast cvt s_5_36 -> [u64; 32]
        let s_5_37: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_5_36);
            buf
        };
        // C s_5_38: const #13776u : u32
        let s_5_38: u32 = 13776;
        // N s_5_39: write-reg s_5_38 <= s_5_37
        let s_5_39: () = {
            state.write_register::<[u64; 32usize]>(s_5_38 as isize, s_5_37);
            tracer.write_register(s_5_38 as isize, s_5_37);
        };
        // N s_5_40: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#20062 <= s_6_0
        fn_state.gs_20062 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
