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
use u__IMPDEF_bits::*;
use set_subrange_zeros::*;
use Zeros::*;
use common::*;
pub fn SPEResetSampleStorage<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_26164: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_1291: i64,
        gs_26180: i64,
        gs_26170: i64,
        i: i64,
        gs_26164: (),
    }
    let fn_state = FunctionState {
        gs_26164,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #32s : i
        let s_0_0: i128 = 32;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u32
        let s_0_2: u32 = (s_0_1.value() as u32);
        // C s_0_3: const #104568u : u32
        let s_0_3: u32 = 104568;
        // N s_0_4: write-reg s_0_3 <= s_0_2
        let s_0_4: () = {
            state.write_register::<u32>(s_0_3 as isize, s_0_2);
            tracer.write_register(s_0_3 as isize, s_0_2);
        };
        // C s_0_5: const #0u : u8
        let s_0_5: bool = false;
        // C s_0_6: const #20160u : u32
        let s_0_6: u32 = 20160;
        // N s_0_7: write-reg s_0_6 <= s_0_5
        let s_0_7: () = {
            state.write_register::<bool>(s_0_6 as isize, s_0_5);
            tracer.write_register(s_0_6 as isize, s_0_5);
        };
        // C s_0_8: const #32s : i
        let s_0_8: i128 = 32;
        // S s_0_9: call Zeros(s_0_8)
        let s_0_9: Bits = Zeros(state, tracer, s_0_8);
        // S s_0_10: cast reint s_0_9 -> u32
        let s_0_10: u32 = (s_0_9.value() as u32);
        // C s_0_11: const #20880u : u32
        let s_0_11: u32 = 20880;
        // N s_0_12: write-reg s_0_11 <= s_0_10
        let s_0_12: () = {
            state.write_register::<u32>(s_0_11 as isize, s_0_10);
            tracer.write_register(s_0_11 as isize, s_0_10);
        };
        // C s_0_13: const #0u : u8
        let s_0_13: bool = false;
        // C s_0_14: const #14880u : u32
        let s_0_14: u32 = 14880;
        // N s_0_15: write-reg s_0_14 <= s_0_13
        let s_0_15: () = {
            state.write_register::<bool>(s_0_14 as isize, s_0_13);
            tracer.write_register(s_0_14 as isize, s_0_13);
        };
        // C s_0_16: const #0s : i64
        let s_0_16: i64 = 0;
        // C s_0_17: const #1s : i
        let s_0_17: i128 = 1;
        // C s_0_18: const #1000u : u32
        let s_0_18: u32 = 1000;
        // D s_0_19: read-reg s_0_18:i64
        let s_0_19: i64 = {
            let value = state.read_register::<i64>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: sub s_0_20 s_0_17
        let s_0_21: i128 = ((s_0_20) - (s_0_17));
        // D s_0_22: cast reint s_0_21 -> i64
        let s_0_22: i64 = (s_0_21 as i64);
        // D s_0_23: write-var gs#26170 <= s_0_22
        fn_state.gs_26170 = s_0_22;
        // D s_0_24: write-var i <= s_0_16
        fn_state.i = s_0_16;
        // N s_0_25: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // D s_1_1: read-var gs#26170:i64
        let s_1_1: i64 = fn_state.gs_26170;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // C s_2_1: const #18464u : u32
        let s_2_1: u32 = 18464;
        // D s_2_2: read-reg s_2_1:[i; 32]
        let s_2_2: [i128; 32usize] = {
            let value = state.read_register::<[i128; 32usize]>(s_2_1 as isize);
            tracer.read_register(s_2_1 as isize, value);
            value
        };
        // D s_2_3: read-var i:i64
        let s_2_3: i64 = fn_state.i;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: mutate-element s_2_2[s_2_4] <= s_2_0
        let s_2_5: [i128; 32usize] = {
            let mut local = s_2_2.clone();
            local[(s_2_4) as usize] = s_2_0;
            local
        };
        // D s_2_6: cast cvt s_2_5 -> [i; 0]
        let s_2_6: alloc::vec::Vec<i128> = alloc::vec::Vec::from(s_2_5);
        // D s_2_7: cast cvt s_2_6 -> [i; 32]
        let s_2_7: [i128; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_2_6);
            buf
        };
        // C s_2_8: const #18464u : u32
        let s_2_8: u32 = 18464;
        // N s_2_9: write-reg s_2_8 <= s_2_7
        let s_2_9: () = {
            state.write_register::<[i128; 32usize]>(s_2_8 as isize, s_2_7);
            tracer.write_register(s_2_8 as isize, s_2_7);
        };
        // C s_2_10: const #23056u : u32
        let s_2_10: u32 = 23056;
        // D s_2_11: read-reg s_2_10:[u8; 32]
        let s_2_11: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_2_10 as isize);
            tracer.read_register(s_2_10 as isize, value);
            value
        };
        // D s_2_12: read-var i:i64
        let s_2_12: i64 = fn_state.i;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // C s_2_14: const #0u : u8
        let s_2_14: bool = false;
        // D s_2_15: mutate-element s_2_11[s_2_13] <= s_2_14
        let s_2_15: [bool; 32usize] = {
            let mut local = s_2_11.clone();
            local[(s_2_13) as usize] = s_2_14;
            local
        };
        // D s_2_16: cast cvt s_2_15 -> [u8; 0]
        let s_2_16: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_2_15);
        // D s_2_17: cast cvt s_2_16 -> [u8; 32]
        let s_2_17: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_2_16);
            buf
        };
        // C s_2_18: const #23056u : u32
        let s_2_18: u32 = 23056;
        // N s_2_19: write-reg s_2_18 <= s_2_17
        let s_2_19: () = {
            state.write_register::<[bool; 32usize]>(s_2_18 as isize, s_2_17);
            tracer.write_register(s_2_18 as isize, s_2_17);
        };
        // C s_2_20: const #102648u : u32
        let s_2_20: u32 = 102648;
        // D s_2_21: read-reg s_2_20:[u8; 32]
        let s_2_21: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_2_20 as isize);
            tracer.read_register(s_2_20 as isize, value);
            value
        };
        // D s_2_22: read-var i:i64
        let s_2_22: i64 = fn_state.i;
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (i128::try_from(s_2_22).unwrap());
        // C s_2_24: const #0u : u8
        let s_2_24: bool = false;
        // D s_2_25: mutate-element s_2_21[s_2_23] <= s_2_24
        let s_2_25: [bool; 32usize] = {
            let mut local = s_2_21.clone();
            local[(s_2_23) as usize] = s_2_24;
            local
        };
        // D s_2_26: cast cvt s_2_25 -> [u8; 0]
        let s_2_26: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_2_25);
        // D s_2_27: cast cvt s_2_26 -> [u8; 32]
        let s_2_27: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_2_26);
            buf
        };
        // C s_2_28: const #102648u : u32
        let s_2_28: u32 = 102648;
        // N s_2_29: write-reg s_2_28 <= s_2_27
        let s_2_29: () = {
            state.write_register::<[bool; 32usize]>(s_2_28 as isize, s_2_27);
            tracer.write_register(s_2_28 as isize, s_2_27);
        };
        // D s_2_30: read-var i:i64
        let s_2_30: i64 = fn_state.i;
        // C s_2_31: const #1s : i64
        let s_2_31: i64 = 1;
        // D s_2_32: add s_2_30 s_2_31
        let s_2_32: i64 = (s_2_30 + s_2_31);
        // D s_2_33: write-var i <= s_2_32
        fn_state.i = s_2_32;
        // N s_2_34: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // C s_3_2: const #992u : u32
        let s_3_2: u32 = 992;
        // D s_3_3: read-reg s_3_2:i64
        let s_3_3: i64 = {
            let value = state.read_register::<i64>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: sub s_3_4 s_3_1
        let s_3_5: i128 = ((s_3_4) - (s_3_1));
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: write-var gs#26180 <= s_3_6
        fn_state.gs_26180 = s_3_6;
        // D s_3_8: write-var u#1291 <= s_3_0
        fn_state.u_1291 = s_3_0;
        // N s_3_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var u#1291:i64
        let s_4_0: i64 = fn_state.u_1291;
        // D s_4_1: read-var gs#26180:i64
        let s_4_1: i64 = fn_state.gs_26180;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #11136u : u32
        let s_5_0: u32 = 11136;
        // D s_5_1: read-reg s_5_0:[u8; 32]
        let s_5_1: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: read-var u#1291:i64
        let s_5_2: i64 = fn_state.u_1291;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // D s_5_5: mutate-element s_5_1[s_5_3] <= s_5_4
        let s_5_5: [bool; 32usize] = {
            let mut local = s_5_1.clone();
            local[(s_5_3) as usize] = s_5_4;
            local
        };
        // D s_5_6: cast cvt s_5_5 -> [u8; 0]
        let s_5_6: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_5_5);
        // D s_5_7: cast cvt s_5_6 -> [u8; 32]
        let s_5_7: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_5_6);
            buf
        };
        // C s_5_8: const #11136u : u32
        let s_5_8: u32 = 11136;
        // N s_5_9: write-reg s_5_8 <= s_5_7
        let s_5_9: () = {
            state.write_register::<[bool; 32usize]>(s_5_8 as isize, s_5_7);
            tracer.write_register(s_5_8 as isize, s_5_7);
        };
        // C s_5_10: const #64s : i
        let s_5_10: i128 = 64;
        // S s_5_11: call Zeros(s_5_10)
        let s_5_11: Bits = Zeros(state, tracer, s_5_10);
        // S s_5_12: cast reint s_5_11 -> u64
        let s_5_12: u64 = (s_5_11.value() as u64);
        // C s_5_13: const #13776u : u32
        let s_5_13: u32 = 13776;
        // D s_5_14: read-reg s_5_13:[u64; 32]
        let s_5_14: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_5_13 as isize);
            tracer.read_register(s_5_13 as isize, value);
            value
        };
        // D s_5_15: read-var u#1291:i64
        let s_5_15: i64 = fn_state.u_1291;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: mutate-element s_5_14[s_5_16] <= s_5_12
        let s_5_17: [u64; 32usize] = {
            let mut local = s_5_14.clone();
            local[(s_5_16) as usize] = s_5_12;
            local
        };
        // D s_5_18: cast cvt s_5_17 -> [u64; 0]
        let s_5_18: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_5_17);
        // D s_5_19: cast cvt s_5_18 -> [u64; 32]
        let s_5_19: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_5_18);
            buf
        };
        // C s_5_20: const #13776u : u32
        let s_5_20: u32 = 13776;
        // N s_5_21: write-reg s_5_20 <= s_5_19
        let s_5_21: () = {
            state.write_register::<[u64; 32usize]>(s_5_20 as isize, s_5_19);
            tracer.write_register(s_5_20 as isize, s_5_19);
        };
        // D s_5_22: read-var u#1291:i64
        let s_5_22: i64 = fn_state.u_1291;
        // C s_5_23: const #1s : i64
        let s_5_23: i64 = 1;
        // D s_5_24: add s_5_22 s_5_23
        let s_5_24: i64 = (s_5_22 + s_5_23);
        // D s_5_25: write-var u#1291 <= s_5_24
        fn_state.u_1291 = s_5_24;
        // N s_5_26: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16s : i
        let s_6_0: i128 = 16;
        // S s_6_1: call Zeros(s_6_0)
        let s_6_1: Bits = Zeros(state, tracer, s_6_0);
        // S s_6_2: cast reint s_6_1 -> u16
        let s_6_2: u16 = (s_6_1.value() as u16);
        // C s_6_3: const #17552u : u32
        let s_6_3: u32 = 17552;
        // N s_6_4: write-reg s_6_3 <= s_6_2
        let s_6_4: () = {
            state.write_register::<u16>(s_6_3 as isize, s_6_2);
            tracer.write_register(s_6_3 as isize, s_6_2);
        };
        // C s_6_5: const #0u : u8
        let s_6_5: bool = false;
        // C s_6_6: const #14184u : u32
        let s_6_6: u32 = 14184;
        // N s_6_7: write-reg s_6_6 <= s_6_5
        let s_6_7: () = {
            state.write_register::<bool>(s_6_6 as isize, s_6_5);
            tracer.write_register(s_6_6 as isize, s_6_5);
        };
        // C s_6_8: const #2s : i
        let s_6_8: i128 = 2;
        // S s_6_9: call Zeros(s_6_8)
        let s_6_9: Bits = Zeros(state, tracer, s_6_8);
        // S s_6_10: cast reint s_6_9 -> u8
        let s_6_10: u8 = (s_6_9.value() as u8);
        // C s_6_11: const #17136u : u32
        let s_6_11: u32 = 17136;
        // N s_6_12: write-reg s_6_11 <= s_6_10
        let s_6_12: () = {
            state.write_register::<u8>(s_6_11 as isize, s_6_10);
            tracer.write_register(s_6_11 as isize, s_6_10);
        };
        // C s_6_13: const #8s : i
        let s_6_13: i128 = 8;
        // S s_6_14: call Zeros(s_6_13)
        let s_6_14: Bits = Zeros(state, tracer, s_6_13);
        // S s_6_15: cast reint s_6_14 -> u8
        let s_6_15: u8 = (s_6_14.value() as u8);
        // C s_6_16: const #13528u : u32
        let s_6_16: u32 = 13528;
        // N s_6_17: write-reg s_6_16 <= s_6_15
        let s_6_17: () = {
            state.write_register::<u8>(s_6_16 as isize, s_6_15);
            tracer.write_register(s_6_16 as isize, s_6_15);
        };
        // C s_6_18: const #0u : u8
        let s_6_18: bool = false;
        // C s_6_19: const #11528u : u32
        let s_6_19: u32 = 11528;
        // N s_6_20: write-reg s_6_19 <= s_6_18
        let s_6_20: () = {
            state.write_register::<bool>(s_6_19 as isize, s_6_18);
            tracer.write_register(s_6_19 as isize, s_6_18);
        };
        // C s_6_21: const #64s : i
        let s_6_21: i128 = 64;
        // S s_6_22: call Zeros(s_6_21)
        let s_6_22: Bits = Zeros(state, tracer, s_6_21);
        // S s_6_23: cast reint s_6_22 -> u64
        let s_6_23: u64 = (s_6_22.value() as u64);
        // C s_6_24: const #13504u : u32
        let s_6_24: u32 = 13504;
        // N s_6_25: write-reg s_6_24 <= s_6_23
        let s_6_25: () = {
            state.write_register::<u64>(s_6_24 as isize, s_6_23);
            tracer.write_register(s_6_24 as isize, s_6_23);
        };
        // C s_6_26: const #0u : u8
        let s_6_26: bool = false;
        // C s_6_27: const #17616u : u32
        let s_6_27: u32 = 17616;
        // N s_6_28: write-reg s_6_27 <= s_6_26
        let s_6_28: () = {
            state.write_register::<bool>(s_6_27 as isize, s_6_26);
            tracer.write_register(s_6_27 as isize, s_6_26);
        };
        // C s_6_29: const #16s : i64
        let s_6_29: i64 = 16;
        // C s_6_30: cast zx s_6_29 -> i
        let s_6_30: i128 = (i128::try_from(s_6_29).unwrap());
        // C s_6_31: const #"SPE EVENTS 63_48" : str
        let s_6_31: &'static str = "SPE EVENTS 63_48";
        // S s_6_32: call __IMPDEF_bits(s_6_30, s_6_31)
        let s_6_32: Bits = u__IMPDEF_bits(state, tracer, s_6_30, s_6_31);
        // S s_6_33: cast reint s_6_32 -> u16
        let s_6_33: u16 = (s_6_32.value() as u16);
        // C s_6_34: const #48s : i
        let s_6_34: i128 = 48;
        // C s_6_35: const #104856u : u32
        let s_6_35: u32 = 104856;
        // D s_6_36: read-reg s_6_35:u64
        let s_6_36: u64 = {
            let value = state.read_register::<u64>(s_6_35 as isize);
            tracer.read_register(s_6_35 as isize, value);
            value
        };
        // D s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 64u16);
        // S s_6_38: cast zx s_6_33 -> bv
        let s_6_38: Bits = Bits::new(s_6_33 as u128, 16u16);
        // C s_6_39: const #15s : i
        let s_6_39: i128 = 15;
        // C s_6_40: const #1u : u64
        let s_6_40: u64 = 1;
        // C s_6_41: cast zx s_6_40 -> bv
        let s_6_41: Bits = Bits::new(s_6_40 as u128, 64u16);
        // C s_6_42: lsl s_6_41 s_6_39
        let s_6_42: Bits = s_6_41 << s_6_39;
        // C s_6_43: sub s_6_42 s_6_41
        let s_6_43: Bits = ((s_6_42) - (s_6_41));
        // S s_6_44: and s_6_38 s_6_43
        let s_6_44: Bits = ((s_6_38) & (s_6_43));
        // S s_6_45: lsl s_6_44 s_6_34
        let s_6_45: Bits = s_6_44 << s_6_34;
        // C s_6_46: lsl s_6_43 s_6_34
        let s_6_46: Bits = s_6_43 << s_6_34;
        // C s_6_47: cmpl s_6_46
        let s_6_47: Bits = !s_6_46;
        // D s_6_48: and s_6_37 s_6_47
        let s_6_48: Bits = ((s_6_37) & (s_6_47));
        // D s_6_49: or s_6_48 s_6_45
        let s_6_49: Bits = ((s_6_48) | (s_6_45));
        // D s_6_50: cast reint s_6_49 -> u64
        let s_6_50: u64 = (s_6_49.value() as u64);
        // C s_6_51: const #104856u : u32
        let s_6_51: u32 = 104856;
        // N s_6_52: write-reg s_6_51 <= s_6_50
        let s_6_52: () = {
            state.write_register::<u64>(s_6_51 as isize, s_6_50);
            tracer.write_register(s_6_51 as isize, s_6_50);
        };
        // C s_6_53: const #64s : i
        let s_6_53: i128 = 64;
        // C s_6_54: const #47s : i
        let s_6_54: i128 = 47;
        // C s_6_55: const #32s : i
        let s_6_55: i128 = 32;
        // C s_6_56: const #104856u : u32
        let s_6_56: u32 = 104856;
        // D s_6_57: read-reg s_6_56:u64
        let s_6_57: u64 = {
            let value = state.read_register::<u64>(s_6_56 as isize);
            tracer.read_register(s_6_56 as isize, value);
            value
        };
        // D s_6_58: cast zx s_6_57 -> bv
        let s_6_58: Bits = Bits::new(s_6_57 as u128, 64u16);
        // D s_6_59: call set_subrange_zeros(s_6_53, s_6_58, s_6_54, s_6_55)
        let s_6_59: Bits = set_subrange_zeros(
            state,
            tracer,
            s_6_53,
            s_6_58,
            s_6_54,
            s_6_55,
        );
        // D s_6_60: cast reint s_6_59 -> u64
        let s_6_60: u64 = (s_6_59.value() as u64);
        // C s_6_61: const #104856u : u32
        let s_6_61: u32 = 104856;
        // N s_6_62: write-reg s_6_61 <= s_6_60
        let s_6_62: () = {
            state.write_register::<u64>(s_6_61 as isize, s_6_60);
            tracer.write_register(s_6_61 as isize, s_6_60);
        };
        // C s_6_63: const #8s : i64
        let s_6_63: i64 = 8;
        // C s_6_64: cast zx s_6_63 -> i
        let s_6_64: i128 = (i128::try_from(s_6_63).unwrap());
        // C s_6_65: const #"SPE EVENTS 31_24" : str
        let s_6_65: &'static str = "SPE EVENTS 31_24";
        // S s_6_66: call __IMPDEF_bits(s_6_64, s_6_65)
        let s_6_66: Bits = u__IMPDEF_bits(state, tracer, s_6_64, s_6_65);
        // S s_6_67: cast reint s_6_66 -> u8
        let s_6_67: u8 = (s_6_66.value() as u8);
        // C s_6_68: const #24s : i
        let s_6_68: i128 = 24;
        // C s_6_69: const #104856u : u32
        let s_6_69: u32 = 104856;
        // D s_6_70: read-reg s_6_69:u64
        let s_6_70: u64 = {
            let value = state.read_register::<u64>(s_6_69 as isize);
            tracer.read_register(s_6_69 as isize, value);
            value
        };
        // D s_6_71: cast zx s_6_70 -> bv
        let s_6_71: Bits = Bits::new(s_6_70 as u128, 64u16);
        // S s_6_72: cast zx s_6_67 -> bv
        let s_6_72: Bits = Bits::new(s_6_67 as u128, 8u16);
        // C s_6_73: const #7s : i
        let s_6_73: i128 = 7;
        // C s_6_74: const #1u : u64
        let s_6_74: u64 = 1;
        // C s_6_75: cast zx s_6_74 -> bv
        let s_6_75: Bits = Bits::new(s_6_74 as u128, 64u16);
        // C s_6_76: lsl s_6_75 s_6_73
        let s_6_76: Bits = s_6_75 << s_6_73;
        // C s_6_77: sub s_6_76 s_6_75
        let s_6_77: Bits = ((s_6_76) - (s_6_75));
        // S s_6_78: and s_6_72 s_6_77
        let s_6_78: Bits = ((s_6_72) & (s_6_77));
        // S s_6_79: lsl s_6_78 s_6_68
        let s_6_79: Bits = s_6_78 << s_6_68;
        // C s_6_80: lsl s_6_77 s_6_68
        let s_6_80: Bits = s_6_77 << s_6_68;
        // C s_6_81: cmpl s_6_80
        let s_6_81: Bits = !s_6_80;
        // D s_6_82: and s_6_71 s_6_81
        let s_6_82: Bits = ((s_6_71) & (s_6_81));
        // D s_6_83: or s_6_82 s_6_79
        let s_6_83: Bits = ((s_6_82) | (s_6_79));
        // D s_6_84: cast reint s_6_83 -> u64
        let s_6_84: u64 = (s_6_83.value() as u64);
        // C s_6_85: const #104856u : u32
        let s_6_85: u32 = 104856;
        // N s_6_86: write-reg s_6_85 <= s_6_84
        let s_6_86: () = {
            state.write_register::<u64>(s_6_85 as isize, s_6_84);
            tracer.write_register(s_6_85 as isize, s_6_84);
        };
        // C s_6_87: const #64s : i
        let s_6_87: i128 = 64;
        // C s_6_88: const #23s : i
        let s_6_88: i128 = 23;
        // C s_6_89: const #16s : i
        let s_6_89: i128 = 16;
        // C s_6_90: const #104856u : u32
        let s_6_90: u32 = 104856;
        // D s_6_91: read-reg s_6_90:u64
        let s_6_91: u64 = {
            let value = state.read_register::<u64>(s_6_90 as isize);
            tracer.read_register(s_6_90 as isize, value);
            value
        };
        // D s_6_92: cast zx s_6_91 -> bv
        let s_6_92: Bits = Bits::new(s_6_91 as u128, 64u16);
        // D s_6_93: call set_subrange_zeros(s_6_87, s_6_92, s_6_88, s_6_89)
        let s_6_93: Bits = set_subrange_zeros(
            state,
            tracer,
            s_6_87,
            s_6_92,
            s_6_88,
            s_6_89,
        );
        // D s_6_94: cast reint s_6_93 -> u64
        let s_6_94: u64 = (s_6_93.value() as u64);
        // C s_6_95: const #104856u : u32
        let s_6_95: u32 = 104856;
        // N s_6_96: write-reg s_6_95 <= s_6_94
        let s_6_96: () = {
            state.write_register::<u64>(s_6_95 as isize, s_6_94);
            tracer.write_register(s_6_95 as isize, s_6_94);
        };
        // C s_6_97: const #4s : i64
        let s_6_97: i64 = 4;
        // C s_6_98: cast zx s_6_97 -> i
        let s_6_98: i128 = (i128::try_from(s_6_97).unwrap());
        // C s_6_99: const #"SPE EVENTS 15_12" : str
        let s_6_99: &'static str = "SPE EVENTS 15_12";
        // S s_6_100: call __IMPDEF_bits(s_6_98, s_6_99)
        let s_6_100: Bits = u__IMPDEF_bits(state, tracer, s_6_98, s_6_99);
        // S s_6_101: cast reint s_6_100 -> u8
        let s_6_101: u8 = (s_6_100.value() as u8);
        // C s_6_102: const #12s : i
        let s_6_102: i128 = 12;
        // C s_6_103: const #104856u : u32
        let s_6_103: u32 = 104856;
        // D s_6_104: read-reg s_6_103:u64
        let s_6_104: u64 = {
            let value = state.read_register::<u64>(s_6_103 as isize);
            tracer.read_register(s_6_103 as isize, value);
            value
        };
        // D s_6_105: cast zx s_6_104 -> bv
        let s_6_105: Bits = Bits::new(s_6_104 as u128, 64u16);
        // S s_6_106: cast zx s_6_101 -> bv
        let s_6_106: Bits = Bits::new(s_6_101 as u128, 4u16);
        // C s_6_107: const #3s : i
        let s_6_107: i128 = 3;
        // C s_6_108: const #1u : u64
        let s_6_108: u64 = 1;
        // C s_6_109: cast zx s_6_108 -> bv
        let s_6_109: Bits = Bits::new(s_6_108 as u128, 64u16);
        // C s_6_110: lsl s_6_109 s_6_107
        let s_6_110: Bits = s_6_109 << s_6_107;
        // C s_6_111: sub s_6_110 s_6_109
        let s_6_111: Bits = ((s_6_110) - (s_6_109));
        // S s_6_112: and s_6_106 s_6_111
        let s_6_112: Bits = ((s_6_106) & (s_6_111));
        // S s_6_113: lsl s_6_112 s_6_102
        let s_6_113: Bits = s_6_112 << s_6_102;
        // C s_6_114: lsl s_6_111 s_6_102
        let s_6_114: Bits = s_6_111 << s_6_102;
        // C s_6_115: cmpl s_6_114
        let s_6_115: Bits = !s_6_114;
        // D s_6_116: and s_6_105 s_6_115
        let s_6_116: Bits = ((s_6_105) & (s_6_115));
        // D s_6_117: or s_6_116 s_6_113
        let s_6_117: Bits = ((s_6_116) | (s_6_113));
        // D s_6_118: cast reint s_6_117 -> u64
        let s_6_118: u64 = (s_6_117.value() as u64);
        // C s_6_119: const #104856u : u32
        let s_6_119: u32 = 104856;
        // N s_6_120: write-reg s_6_119 <= s_6_118
        let s_6_120: () = {
            state.write_register::<u64>(s_6_119 as isize, s_6_118);
            tracer.write_register(s_6_119 as isize, s_6_118);
        };
        // C s_6_121: const #64s : i
        let s_6_121: i128 = 64;
        // C s_6_122: const #11s : i
        let s_6_122: i128 = 11;
        // C s_6_123: const #0s : i
        let s_6_123: i128 = 0;
        // C s_6_124: const #104856u : u32
        let s_6_124: u32 = 104856;
        // D s_6_125: read-reg s_6_124:u64
        let s_6_125: u64 = {
            let value = state.read_register::<u64>(s_6_124 as isize);
            tracer.read_register(s_6_124 as isize, value);
            value
        };
        // D s_6_126: cast zx s_6_125 -> bv
        let s_6_126: Bits = Bits::new(s_6_125 as u128, 64u16);
        // D s_6_127: call set_subrange_zeros(s_6_121, s_6_126, s_6_122, s_6_123)
        let s_6_127: Bits = set_subrange_zeros(
            state,
            tracer,
            s_6_121,
            s_6_126,
            s_6_122,
            s_6_123,
        );
        // D s_6_128: cast reint s_6_127 -> u64
        let s_6_128: u64 = (s_6_127.value() as u64);
        // C s_6_129: const #104856u : u32
        let s_6_129: u32 = 104856;
        // N s_6_130: write-reg s_6_129 <= s_6_128
        let s_6_130: () = {
            state.write_register::<u64>(s_6_129 as isize, s_6_128);
            tracer.write_register(s_6_129 as isize, s_6_128);
        };
        // C s_6_131: const #0u : u8
        let s_6_131: bool = false;
        // C s_6_132: const #10528u : u32
        let s_6_132: u32 = 10528;
        // N s_6_133: write-reg s_6_132 <= s_6_131
        let s_6_133: () = {
            state.write_register::<bool>(s_6_132 as isize, s_6_131);
            tracer.write_register(s_6_132 as isize, s_6_131);
        };
        // N s_6_134: return
        return;
    }
}
