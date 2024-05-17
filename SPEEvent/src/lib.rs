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
use HaveStatisticalProfilingv1p4::*;
use Bit::*;
use HaveStatisticalProfilingv1p1::*;
use common::*;
pub fn SPEEvent<T: Tracer>(state: &mut State, tracer: &T, pmuevent: u16) -> () {
    #[derive(Default)]
    struct FunctionState {
        pmuevent: u16,
    }
    let fn_state = FunctionState {
        pmuevent,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var pmuevent:u16
        let s_0_0: u16 = fn_state.pmuevent;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 16u16);
        // C s_0_2: const #280u : u32
        let s_0_2: u32 = 280;
        // D s_0_3: read-reg s_0_2:u16
        let s_0_3: u16 = {
            let value = state.read_register::<u16>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 16u16);
        // D s_0_5: cmp-eq s_0_1 s_0_4
        let s_0_5: bool = ((s_0_1) == (s_0_4));
        // D s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: branch s_0_6 b4 b1
        if s_0_6 {
            return block_4(state, tracer, fn_state);
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
        // S s_1_1: call HaveStatisticalProfilingv1p4(s_1_0)
        let s_1_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b3 b2
        if s_1_1 {
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
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // S s_3_1: call Bit(s_3_0)
        let s_3_1: bool = Bit(state, tracer, s_3_0);
        // C s_3_2: const #23s : i
        let s_3_2: i128 = 23;
        // C s_3_3: const #104856u : u32
        let s_3_3: u32 = 104856;
        // D s_3_4: read-reg s_3_3:u64
        let s_3_4: u64 = {
            let value = state.read_register::<u64>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 64u16);
        // C s_3_6: const #1u : u64
        let s_3_6: u64 = 1;
        // D s_3_7: bit-insert s_3_5 s_3_5 s_3_2 s_3_6
        let s_3_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_3_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_3_5.length(),
            );
            (s_3_5 & mask) | (s_3_5 << s_3_2)
        };
        // D s_3_8: cast reint s_3_7 -> u64
        let s_3_8: u64 = (s_3_7.value() as u64);
        // C s_3_9: const #104856u : u32
        let s_3_9: u32 = 104856;
        // N s_3_10: write-reg s_3_9 <= s_3_8
        let s_3_10: () = {
            state.write_register::<u64>(s_3_9 as isize, s_3_8);
            tracer.write_register(s_3_9 as isize, s_3_8);
        };
        // N s_3_11: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var pmuevent:u16
        let s_4_0: u16 = fn_state.pmuevent;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 16u16);
        // C s_4_2: const #320u : u32
        let s_4_2: u32 = 320;
        // D s_4_3: read-reg s_4_2:u16
        let s_4_3: u16 = {
            let value = state.read_register::<u16>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 16u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // D s_4_6: not s_4_5
        let s_4_6: bool = !s_4_5;
        // N s_4_7: branch s_4_6 b8 b5
        if s_4_6 {
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveStatisticalProfilingv1p4(s_5_0)
        let s_5_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b7 b6
        if s_5_1 {
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
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // S s_7_1: call Bit(s_7_0)
        let s_7_1: bool = Bit(state, tracer, s_7_0);
        // C s_7_2: const #22s : i
        let s_7_2: i128 = 22;
        // C s_7_3: const #104856u : u32
        let s_7_3: u32 = 104856;
        // D s_7_4: read-reg s_7_3:u64
        let s_7_4: u64 = {
            let value = state.read_register::<u64>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 64u16);
        // C s_7_6: const #1u : u64
        let s_7_6: u64 = 1;
        // D s_7_7: bit-insert s_7_5 s_7_5 s_7_2 s_7_6
        let s_7_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_7_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_7_5.length(),
            );
            (s_7_5 & mask) | (s_7_5 << s_7_2)
        };
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // C s_7_9: const #104856u : u32
        let s_7_9: u32 = 104856;
        // N s_7_10: write-reg s_7_9 <= s_7_8
        let s_7_10: () = {
            state.write_register::<u64>(s_7_9 as isize, s_7_8);
            tracer.write_register(s_7_9 as isize, s_7_8);
        };
        // N s_7_11: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var pmuevent:u16
        let s_8_0: u16 = fn_state.pmuevent;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 16u16);
        // C s_8_2: const #328u : u32
        let s_8_2: u32 = 328;
        // D s_8_3: read-reg s_8_2:u16
        let s_8_3: u16 = {
            let value = state.read_register::<u16>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 16u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // D s_8_6: not s_8_5
        let s_8_6: bool = !s_8_5;
        // N s_8_7: branch s_8_6 b12 b9
        if s_8_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HaveStatisticalProfilingv1p4(s_9_0)
        let s_9_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b11 b10
        if s_9_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // S s_11_1: call Bit(s_11_0)
        let s_11_1: bool = Bit(state, tracer, s_11_0);
        // C s_11_2: const #22s : i
        let s_11_2: i128 = 22;
        // C s_11_3: const #104856u : u32
        let s_11_3: u32 = 104856;
        // D s_11_4: read-reg s_11_3:u64
        let s_11_4: u64 = {
            let value = state.read_register::<u64>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 64u16);
        // C s_11_6: const #1u : u64
        let s_11_6: u64 = 1;
        // D s_11_7: bit-insert s_11_5 s_11_5 s_11_2 s_11_6
        let s_11_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_11_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_11_5.length(),
            );
            (s_11_5 & mask) | (s_11_5 << s_11_2)
        };
        // D s_11_8: cast reint s_11_7 -> u64
        let s_11_8: u64 = (s_11_7.value() as u64);
        // C s_11_9: const #104856u : u32
        let s_11_9: u32 = 104856;
        // N s_11_10: write-reg s_11_9 <= s_11_8
        let s_11_10: () = {
            state.write_register::<u64>(s_11_9 as isize, s_11_8);
            tracer.write_register(s_11_9 as isize, s_11_8);
        };
        // N s_11_11: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var pmuevent:u16
        let s_12_0: u16 = fn_state.pmuevent;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 16u16);
        // C s_12_2: const #336u : u32
        let s_12_2: u32 = 336;
        // D s_12_3: read-reg s_12_2:u16
        let s_12_3: u16 = {
            let value = state.read_register::<u16>(s_12_2 as isize);
            tracer.read_register(s_12_2 as isize, value);
            value
        };
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 16u16);
        // D s_12_5: cmp-eq s_12_1 s_12_4
        let s_12_5: bool = ((s_12_1) == (s_12_4));
        // D s_12_6: not s_12_5
        let s_12_6: bool = !s_12_5;
        // N s_12_7: branch s_12_6 b16 b13
        if s_12_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HaveStatisticalProfilingv1p4(s_13_0)
        let s_13_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b15 b14
        if s_13_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // S s_15_1: call Bit(s_15_0)
        let s_15_1: bool = Bit(state, tracer, s_15_0);
        // C s_15_2: const #22s : i
        let s_15_2: i128 = 22;
        // C s_15_3: const #104856u : u32
        let s_15_3: u32 = 104856;
        // D s_15_4: read-reg s_15_3:u64
        let s_15_4: u64 = {
            let value = state.read_register::<u64>(s_15_3 as isize);
            tracer.read_register(s_15_3 as isize, value);
            value
        };
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 64u16);
        // C s_15_6: const #1u : u64
        let s_15_6: u64 = 1;
        // D s_15_7: bit-insert s_15_5 s_15_5 s_15_2 s_15_6
        let s_15_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_15_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_15_5.length(),
            );
            (s_15_5 & mask) | (s_15_5 << s_15_2)
        };
        // D s_15_8: cast reint s_15_7 -> u64
        let s_15_8: u64 = (s_15_7.value() as u64);
        // C s_15_9: const #104856u : u32
        let s_15_9: u32 = 104856;
        // N s_15_10: write-reg s_15_9 <= s_15_8
        let s_15_10: () = {
            state.write_register::<u64>(s_15_9 as isize, s_15_8);
            tracer.write_register(s_15_9 as isize, s_15_8);
        };
        // N s_15_11: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var pmuevent:u16
        let s_16_0: u16 = fn_state.pmuevent;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 16u16);
        // C s_16_2: const #344u : u32
        let s_16_2: u32 = 344;
        // D s_16_3: read-reg s_16_2:u16
        let s_16_3: u16 = {
            let value = state.read_register::<u16>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 16u16);
        // D s_16_5: cmp-eq s_16_1 s_16_4
        let s_16_5: bool = ((s_16_1) == (s_16_4));
        // D s_16_6: not s_16_5
        let s_16_6: bool = !s_16_5;
        // N s_16_7: branch s_16_6 b20 b17
        if s_16_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HaveStatisticalProfilingv1p4(s_17_0)
        let s_17_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b19 b18
        if s_17_1 {
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
        // S s_19_1: call Bit(s_19_0)
        let s_19_1: bool = Bit(state, tracer, s_19_0);
        // C s_19_2: const #22s : i
        let s_19_2: i128 = 22;
        // C s_19_3: const #104856u : u32
        let s_19_3: u32 = 104856;
        // D s_19_4: read-reg s_19_3:u64
        let s_19_4: u64 = {
            let value = state.read_register::<u64>(s_19_3 as isize);
            tracer.read_register(s_19_3 as isize, value);
            value
        };
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 64u16);
        // C s_19_6: const #1u : u64
        let s_19_6: u64 = 1;
        // D s_19_7: bit-insert s_19_5 s_19_5 s_19_2 s_19_6
        let s_19_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_19_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_19_5.length(),
            );
            (s_19_5 & mask) | (s_19_5 << s_19_2)
        };
        // D s_19_8: cast reint s_19_7 -> u64
        let s_19_8: u64 = (s_19_7.value() as u64);
        // C s_19_9: const #104856u : u32
        let s_19_9: u32 = 104856;
        // N s_19_10: write-reg s_19_9 <= s_19_8
        let s_19_10: () = {
            state.write_register::<u64>(s_19_9 as isize, s_19_8);
            tracer.write_register(s_19_9 as isize, s_19_8);
        };
        // N s_19_11: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var pmuevent:u16
        let s_20_0: u16 = fn_state.pmuevent;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 16u16);
        // C s_20_2: const #288u : u32
        let s_20_2: u32 = 288;
        // D s_20_3: read-reg s_20_2:u16
        let s_20_3: u16 = {
            let value = state.read_register::<u16>(s_20_2 as isize);
            tracer.read_register(s_20_2 as isize, value);
            value
        };
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 16u16);
        // D s_20_5: cmp-eq s_20_1 s_20_4
        let s_20_5: bool = ((s_20_1) == (s_20_4));
        // D s_20_6: not s_20_5
        let s_20_6: bool = !s_20_5;
        // N s_20_7: branch s_20_6 b24 b21
        if s_20_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HaveStatisticalProfilingv1p4(s_21_0)
        let s_21_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b23 b22
        if s_21_1 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // S s_23_1: call Bit(s_23_0)
        let s_23_1: bool = Bit(state, tracer, s_23_0);
        // C s_23_2: const #21s : i
        let s_23_2: i128 = 21;
        // C s_23_3: const #104856u : u32
        let s_23_3: u32 = 104856;
        // D s_23_4: read-reg s_23_3:u64
        let s_23_4: u64 = {
            let value = state.read_register::<u64>(s_23_3 as isize);
            tracer.read_register(s_23_3 as isize, value);
            value
        };
        // D s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 64u16);
        // C s_23_6: const #1u : u64
        let s_23_6: u64 = 1;
        // D s_23_7: bit-insert s_23_5 s_23_5 s_23_2 s_23_6
        let s_23_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_23_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_23_5.length(),
            );
            (s_23_5 & mask) | (s_23_5 << s_23_2)
        };
        // D s_23_8: cast reint s_23_7 -> u64
        let s_23_8: u64 = (s_23_7.value() as u64);
        // C s_23_9: const #104856u : u32
        let s_23_9: u32 = 104856;
        // N s_23_10: write-reg s_23_9 <= s_23_8
        let s_23_10: () = {
            state.write_register::<u64>(s_23_9 as isize, s_23_8);
            tracer.write_register(s_23_9 as isize, s_23_8);
        };
        // N s_23_11: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var pmuevent:u16
        let s_24_0: u16 = fn_state.pmuevent;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 16u16);
        // C s_24_2: const #296u : u32
        let s_24_2: u32 = 296;
        // D s_24_3: read-reg s_24_2:u16
        let s_24_3: u16 = {
            let value = state.read_register::<u16>(s_24_2 as isize);
            tracer.read_register(s_24_2 as isize, value);
            value
        };
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 16u16);
        // D s_24_5: cmp-eq s_24_1 s_24_4
        let s_24_5: bool = ((s_24_1) == (s_24_4));
        // D s_24_6: not s_24_5
        let s_24_6: bool = !s_24_5;
        // N s_24_7: branch s_24_6 b28 b25
        if s_24_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HaveStatisticalProfilingv1p4(s_25_0)
        let s_25_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b27 b26
        if s_25_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // S s_27_1: call Bit(s_27_0)
        let s_27_1: bool = Bit(state, tracer, s_27_0);
        // C s_27_2: const #21s : i
        let s_27_2: i128 = 21;
        // C s_27_3: const #104856u : u32
        let s_27_3: u32 = 104856;
        // D s_27_4: read-reg s_27_3:u64
        let s_27_4: u64 = {
            let value = state.read_register::<u64>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // D s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 64u16);
        // C s_27_6: const #1u : u64
        let s_27_6: u64 = 1;
        // D s_27_7: bit-insert s_27_5 s_27_5 s_27_2 s_27_6
        let s_27_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_27_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_27_5.length(),
            );
            (s_27_5 & mask) | (s_27_5 << s_27_2)
        };
        // D s_27_8: cast reint s_27_7 -> u64
        let s_27_8: u64 = (s_27_7.value() as u64);
        // C s_27_9: const #104856u : u32
        let s_27_9: u32 = 104856;
        // N s_27_10: write-reg s_27_9 <= s_27_8
        let s_27_10: () = {
            state.write_register::<u64>(s_27_9 as isize, s_27_8);
            tracer.write_register(s_27_9 as isize, s_27_8);
        };
        // N s_27_11: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var pmuevent:u16
        let s_28_0: u16 = fn_state.pmuevent;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 16u16);
        // C s_28_2: const #304u : u32
        let s_28_2: u32 = 304;
        // D s_28_3: read-reg s_28_2:u16
        let s_28_3: u16 = {
            let value = state.read_register::<u16>(s_28_2 as isize);
            tracer.read_register(s_28_2 as isize, value);
            value
        };
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 16u16);
        // D s_28_5: cmp-eq s_28_1 s_28_4
        let s_28_5: bool = ((s_28_1) == (s_28_4));
        // D s_28_6: not s_28_5
        let s_28_6: bool = !s_28_5;
        // N s_28_7: branch s_28_6 b32 b29
        if s_28_6 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call HaveStatisticalProfilingv1p4(s_29_0)
        let s_29_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b31 b30
        if s_29_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // S s_31_1: call Bit(s_31_0)
        let s_31_1: bool = Bit(state, tracer, s_31_0);
        // C s_31_2: const #21s : i
        let s_31_2: i128 = 21;
        // C s_31_3: const #104856u : u32
        let s_31_3: u32 = 104856;
        // D s_31_4: read-reg s_31_3:u64
        let s_31_4: u64 = {
            let value = state.read_register::<u64>(s_31_3 as isize);
            tracer.read_register(s_31_3 as isize, value);
            value
        };
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 64u16);
        // C s_31_6: const #1u : u64
        let s_31_6: u64 = 1;
        // D s_31_7: bit-insert s_31_5 s_31_5 s_31_2 s_31_6
        let s_31_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_31_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_31_5.length(),
            );
            (s_31_5 & mask) | (s_31_5 << s_31_2)
        };
        // D s_31_8: cast reint s_31_7 -> u64
        let s_31_8: u64 = (s_31_7.value() as u64);
        // C s_31_9: const #104856u : u32
        let s_31_9: u32 = 104856;
        // N s_31_10: write-reg s_31_9 <= s_31_8
        let s_31_10: () = {
            state.write_register::<u64>(s_31_9 as isize, s_31_8);
            tracer.write_register(s_31_9 as isize, s_31_8);
        };
        // N s_31_11: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var pmuevent:u16
        let s_32_0: u16 = fn_state.pmuevent;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 16u16);
        // C s_32_2: const #312u : u32
        let s_32_2: u32 = 312;
        // D s_32_3: read-reg s_32_2:u16
        let s_32_3: u16 = {
            let value = state.read_register::<u16>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // D s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 16u16);
        // D s_32_5: cmp-eq s_32_1 s_32_4
        let s_32_5: bool = ((s_32_1) == (s_32_4));
        // D s_32_6: not s_32_5
        let s_32_6: bool = !s_32_5;
        // N s_32_7: branch s_32_6 b36 b33
        if s_32_6 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call HaveStatisticalProfilingv1p4(s_33_0)
        let s_33_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b35 b34
        if s_33_1 {
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
        // N s_34_0: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // S s_35_1: call Bit(s_35_0)
        let s_35_1: bool = Bit(state, tracer, s_35_0);
        // C s_35_2: const #21s : i
        let s_35_2: i128 = 21;
        // C s_35_3: const #104856u : u32
        let s_35_3: u32 = 104856;
        // D s_35_4: read-reg s_35_3:u64
        let s_35_4: u64 = {
            let value = state.read_register::<u64>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // D s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 64u16);
        // C s_35_6: const #1u : u64
        let s_35_6: u64 = 1;
        // D s_35_7: bit-insert s_35_5 s_35_5 s_35_2 s_35_6
        let s_35_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_35_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_35_5.length(),
            );
            (s_35_5 & mask) | (s_35_5 << s_35_2)
        };
        // D s_35_8: cast reint s_35_7 -> u64
        let s_35_8: u64 = (s_35_7.value() as u64);
        // C s_35_9: const #104856u : u32
        let s_35_9: u32 = 104856;
        // N s_35_10: write-reg s_35_9 <= s_35_8
        let s_35_10: () = {
            state.write_register::<u64>(s_35_9 as isize, s_35_8);
            tracer.write_register(s_35_9 as isize, s_35_8);
        };
        // N s_35_11: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var pmuevent:u16
        let s_36_0: u16 = fn_state.pmuevent;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 16u16);
        // C s_36_2: const #184u : u32
        let s_36_2: u32 = 184;
        // D s_36_3: read-reg s_36_2:u16
        let s_36_3: u16 = {
            let value = state.read_register::<u16>(s_36_2 as isize);
            tracer.read_register(s_36_2 as isize, value);
            value
        };
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 16u16);
        // D s_36_5: cmp-eq s_36_1 s_36_4
        let s_36_5: bool = ((s_36_1) == (s_36_4));
        // D s_36_6: not s_36_5
        let s_36_6: bool = !s_36_5;
        // N s_36_7: branch s_36_6 b40 b37
        if s_36_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call HaveStatisticalProfilingv1p4(s_37_0)
        let s_37_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b39 b38
        if s_37_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // S s_39_1: call Bit(s_39_0)
        let s_39_1: bool = Bit(state, tracer, s_39_0);
        // C s_39_2: const #20s : i
        let s_39_2: i128 = 20;
        // C s_39_3: const #104856u : u32
        let s_39_3: u32 = 104856;
        // D s_39_4: read-reg s_39_3:u64
        let s_39_4: u64 = {
            let value = state.read_register::<u64>(s_39_3 as isize);
            tracer.read_register(s_39_3 as isize, value);
            value
        };
        // D s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 64u16);
        // C s_39_6: const #1u : u64
        let s_39_6: u64 = 1;
        // D s_39_7: bit-insert s_39_5 s_39_5 s_39_2 s_39_6
        let s_39_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_39_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_39_5.length(),
            );
            (s_39_5 & mask) | (s_39_5 << s_39_2)
        };
        // D s_39_8: cast reint s_39_7 -> u64
        let s_39_8: u64 = (s_39_7.value() as u64);
        // C s_39_9: const #104856u : u32
        let s_39_9: u32 = 104856;
        // N s_39_10: write-reg s_39_9 <= s_39_8
        let s_39_10: () = {
            state.write_register::<u64>(s_39_9 as isize, s_39_8);
            tracer.write_register(s_39_9 as isize, s_39_8);
        };
        // N s_39_11: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var pmuevent:u16
        let s_40_0: u16 = fn_state.pmuevent;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 16u16);
        // C s_40_2: const #144u : u32
        let s_40_2: u32 = 144;
        // D s_40_3: read-reg s_40_2:u16
        let s_40_3: u16 = {
            let value = state.read_register::<u16>(s_40_2 as isize);
            tracer.read_register(s_40_2 as isize, value);
            value
        };
        // D s_40_4: cast zx s_40_3 -> bv
        let s_40_4: Bits = Bits::new(s_40_3 as u128, 16u16);
        // D s_40_5: cmp-eq s_40_1 s_40_4
        let s_40_5: bool = ((s_40_1) == (s_40_4));
        // D s_40_6: not s_40_5
        let s_40_6: bool = !s_40_5;
        // N s_40_7: branch s_40_6 b44 b41
        if s_40_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call HaveStatisticalProfilingv1p4(s_41_0)
        let s_41_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b43 b42
        if s_41_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // S s_43_1: call Bit(s_43_0)
        let s_43_1: bool = Bit(state, tracer, s_43_0);
        // C s_43_2: const #19s : i
        let s_43_2: i128 = 19;
        // C s_43_3: const #104856u : u32
        let s_43_3: u32 = 104856;
        // D s_43_4: read-reg s_43_3:u64
        let s_43_4: u64 = {
            let value = state.read_register::<u64>(s_43_3 as isize);
            tracer.read_register(s_43_3 as isize, value);
            value
        };
        // D s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 64u16);
        // C s_43_6: const #1u : u64
        let s_43_6: u64 = 1;
        // D s_43_7: bit-insert s_43_5 s_43_5 s_43_2 s_43_6
        let s_43_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_43_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_43_5.length(),
            );
            (s_43_5 & mask) | (s_43_5 << s_43_2)
        };
        // D s_43_8: cast reint s_43_7 -> u64
        let s_43_8: u64 = (s_43_7.value() as u64);
        // C s_43_9: const #104856u : u32
        let s_43_9: u32 = 104856;
        // N s_43_10: write-reg s_43_9 <= s_43_8
        let s_43_10: () = {
            state.write_register::<u64>(s_43_9 as isize, s_43_8);
            tracer.write_register(s_43_9 as isize, s_43_8);
        };
        // N s_43_11: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var pmuevent:u16
        let s_44_0: u16 = fn_state.pmuevent;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 16u16);
        // C s_44_2: const #200u : u32
        let s_44_2: u32 = 200;
        // D s_44_3: read-reg s_44_2:u16
        let s_44_3: u16 = {
            let value = state.read_register::<u16>(s_44_2 as isize);
            tracer.read_register(s_44_2 as isize, value);
            value
        };
        // D s_44_4: cast zx s_44_3 -> bv
        let s_44_4: Bits = Bits::new(s_44_3 as u128, 16u16);
        // D s_44_5: cmp-eq s_44_1 s_44_4
        let s_44_5: bool = ((s_44_1) == (s_44_4));
        // D s_44_6: not s_44_5
        let s_44_6: bool = !s_44_5;
        // N s_44_7: branch s_44_6 b48 b45
        if s_44_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call HaveStatisticalProfilingv1p1(s_45_0)
        let s_45_1: bool = HaveStatisticalProfilingv1p1(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b47 b46
        if s_45_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // S s_47_1: call Bit(s_47_0)
        let s_47_1: bool = Bit(state, tracer, s_47_0);
        // C s_47_2: const #18s : i
        let s_47_2: i128 = 18;
        // C s_47_3: const #104856u : u32
        let s_47_3: u32 = 104856;
        // D s_47_4: read-reg s_47_3:u64
        let s_47_4: u64 = {
            let value = state.read_register::<u64>(s_47_3 as isize);
            tracer.read_register(s_47_3 as isize, value);
            value
        };
        // D s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 64u16);
        // C s_47_6: const #1u : u64
        let s_47_6: u64 = 1;
        // D s_47_7: bit-insert s_47_5 s_47_5 s_47_2 s_47_6
        let s_47_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_47_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_47_5.length(),
            );
            (s_47_5 & mask) | (s_47_5 << s_47_2)
        };
        // D s_47_8: cast reint s_47_7 -> u64
        let s_47_8: u64 = (s_47_7.value() as u64);
        // C s_47_9: const #104856u : u32
        let s_47_9: u32 = 104856;
        // N s_47_10: write-reg s_47_9 <= s_47_8
        let s_47_10: () = {
            state.write_register::<u64>(s_47_9 as isize, s_47_8);
            tracer.write_register(s_47_9 as isize, s_47_8);
        };
        // N s_47_11: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var pmuevent:u16
        let s_48_0: u16 = fn_state.pmuevent;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 16u16);
        // C s_48_2: const #208u : u32
        let s_48_2: u32 = 208;
        // D s_48_3: read-reg s_48_2:u16
        let s_48_3: u16 = {
            let value = state.read_register::<u16>(s_48_2 as isize);
            tracer.read_register(s_48_2 as isize, value);
            value
        };
        // D s_48_4: cast zx s_48_3 -> bv
        let s_48_4: Bits = Bits::new(s_48_3 as u128, 16u16);
        // D s_48_5: cmp-eq s_48_1 s_48_4
        let s_48_5: bool = ((s_48_1) == (s_48_4));
        // D s_48_6: not s_48_5
        let s_48_6: bool = !s_48_5;
        // N s_48_7: branch s_48_6 b52 b49
        if s_48_6 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call HaveStatisticalProfilingv1p1(s_49_0)
        let s_49_1: bool = HaveStatisticalProfilingv1p1(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b51 b50
        if s_49_1 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // S s_51_1: call Bit(s_51_0)
        let s_51_1: bool = Bit(state, tracer, s_51_0);
        // C s_51_2: const #17s : i
        let s_51_2: i128 = 17;
        // C s_51_3: const #104856u : u32
        let s_51_3: u32 = 104856;
        // D s_51_4: read-reg s_51_3:u64
        let s_51_4: u64 = {
            let value = state.read_register::<u64>(s_51_3 as isize);
            tracer.read_register(s_51_3 as isize, value);
            value
        };
        // D s_51_5: cast zx s_51_4 -> bv
        let s_51_5: Bits = Bits::new(s_51_4 as u128, 64u16);
        // C s_51_6: const #1u : u64
        let s_51_6: u64 = 1;
        // D s_51_7: bit-insert s_51_5 s_51_5 s_51_2 s_51_6
        let s_51_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_51_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_51_5.length(),
            );
            (s_51_5 & mask) | (s_51_5 << s_51_2)
        };
        // D s_51_8: cast reint s_51_7 -> u64
        let s_51_8: u64 = (s_51_7.value() as u64);
        // C s_51_9: const #104856u : u32
        let s_51_9: u32 = 104856;
        // N s_51_10: write-reg s_51_9 <= s_51_8
        let s_51_10: () = {
            state.write_register::<u64>(s_51_9 as isize, s_51_8);
            tracer.write_register(s_51_9 as isize, s_51_8);
        };
        // N s_51_11: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var pmuevent:u16
        let s_52_0: u16 = fn_state.pmuevent;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 16u16);
        // C s_52_2: const #192u : u32
        let s_52_2: u32 = 192;
        // D s_52_3: read-reg s_52_2:u16
        let s_52_3: u16 = {
            let value = state.read_register::<u16>(s_52_2 as isize);
            tracer.read_register(s_52_2 as isize, value);
            value
        };
        // D s_52_4: cast zx s_52_3 -> bv
        let s_52_4: Bits = Bits::new(s_52_3 as u128, 16u16);
        // D s_52_5: cmp-eq s_52_1 s_52_4
        let s_52_5: bool = ((s_52_1) == (s_52_4));
        // D s_52_6: not s_52_5
        let s_52_6: bool = !s_52_5;
        // N s_52_7: branch s_52_6 b56 b53
        if s_52_6 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call HaveStatisticalProfilingv1p1(s_53_0)
        let s_53_1: bool = HaveStatisticalProfilingv1p1(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b55 b54
        if s_53_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // S s_55_1: call Bit(s_55_0)
        let s_55_1: bool = Bit(state, tracer, s_55_0);
        // C s_55_2: const #11s : i
        let s_55_2: i128 = 11;
        // C s_55_3: const #104856u : u32
        let s_55_3: u32 = 104856;
        // D s_55_4: read-reg s_55_3:u64
        let s_55_4: u64 = {
            let value = state.read_register::<u64>(s_55_3 as isize);
            tracer.read_register(s_55_3 as isize, value);
            value
        };
        // D s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 64u16);
        // C s_55_6: const #1u : u64
        let s_55_6: u64 = 1;
        // D s_55_7: bit-insert s_55_5 s_55_5 s_55_2 s_55_6
        let s_55_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_55_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_55_5.length(),
            );
            (s_55_5 & mask) | (s_55_5 << s_55_2)
        };
        // D s_55_8: cast reint s_55_7 -> u64
        let s_55_8: u64 = (s_55_7.value() as u64);
        // C s_55_9: const #104856u : u32
        let s_55_9: u32 = 104856;
        // N s_55_10: write-reg s_55_9 <= s_55_8
        let s_55_10: () = {
            state.write_register::<u64>(s_55_9 as isize, s_55_8);
            tracer.write_register(s_55_9 as isize, s_55_8);
        };
        // N s_55_11: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var pmuevent:u16
        let s_56_0: u16 = fn_state.pmuevent;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 16u16);
        // C s_56_2: const #104u : u32
        let s_56_2: u32 = 104;
        // D s_56_3: read-reg s_56_2:u16
        let s_56_3: u16 = {
            let value = state.read_register::<u16>(s_56_2 as isize);
            tracer.read_register(s_56_2 as isize, value);
            value
        };
        // D s_56_4: cast zx s_56_3 -> bv
        let s_56_4: Bits = Bits::new(s_56_3 as u128, 16u16);
        // D s_56_5: cmp-eq s_56_1 s_56_4
        let s_56_5: bool = ((s_56_1) == (s_56_4));
        // D s_56_6: not s_56_5
        let s_56_6: bool = !s_56_5;
        // N s_56_7: branch s_56_6 b58 b57
        if s_56_6 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // S s_57_1: call Bit(s_57_0)
        let s_57_1: bool = Bit(state, tracer, s_57_0);
        // C s_57_2: const #10s : i
        let s_57_2: i128 = 10;
        // C s_57_3: const #104856u : u32
        let s_57_3: u32 = 104856;
        // D s_57_4: read-reg s_57_3:u64
        let s_57_4: u64 = {
            let value = state.read_register::<u64>(s_57_3 as isize);
            tracer.read_register(s_57_3 as isize, value);
            value
        };
        // D s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 64u16);
        // C s_57_6: const #1u : u64
        let s_57_6: u64 = 1;
        // D s_57_7: bit-insert s_57_5 s_57_5 s_57_2 s_57_6
        let s_57_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_57_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_57_5.length(),
            );
            (s_57_5 & mask) | (s_57_5 << s_57_2)
        };
        // D s_57_8: cast reint s_57_7 -> u64
        let s_57_8: u64 = (s_57_7.value() as u64);
        // C s_57_9: const #104856u : u32
        let s_57_9: u32 = 104856;
        // N s_57_10: write-reg s_57_9 <= s_57_8
        let s_57_10: () = {
            state.write_register::<u64>(s_57_9 as isize, s_57_8);
            tracer.write_register(s_57_9 as isize, s_57_8);
        };
        // N s_57_11: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var pmuevent:u16
        let s_58_0: u16 = fn_state.pmuevent;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 16u16);
        // C s_58_2: const #120u : u32
        let s_58_2: u32 = 120;
        // D s_58_3: read-reg s_58_2:u16
        let s_58_3: u16 = {
            let value = state.read_register::<u16>(s_58_2 as isize);
            tracer.read_register(s_58_2 as isize, value);
            value
        };
        // D s_58_4: cast zx s_58_3 -> bv
        let s_58_4: Bits = Bits::new(s_58_3 as u128, 16u16);
        // D s_58_5: cmp-eq s_58_1 s_58_4
        let s_58_5: bool = ((s_58_1) == (s_58_4));
        // D s_58_6: not s_58_5
        let s_58_6: bool = !s_58_5;
        // N s_58_7: branch s_58_6 b60 b59
        if s_58_6 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // S s_59_1: call Bit(s_59_0)
        let s_59_1: bool = Bit(state, tracer, s_59_0);
        // C s_59_2: const #9s : i
        let s_59_2: i128 = 9;
        // C s_59_3: const #104856u : u32
        let s_59_3: u32 = 104856;
        // D s_59_4: read-reg s_59_3:u64
        let s_59_4: u64 = {
            let value = state.read_register::<u64>(s_59_3 as isize);
            tracer.read_register(s_59_3 as isize, value);
            value
        };
        // D s_59_5: cast zx s_59_4 -> bv
        let s_59_5: Bits = Bits::new(s_59_4 as u128, 64u16);
        // C s_59_6: const #1u : u64
        let s_59_6: u64 = 1;
        // D s_59_7: bit-insert s_59_5 s_59_5 s_59_2 s_59_6
        let s_59_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_59_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_59_5.length(),
            );
            (s_59_5 & mask) | (s_59_5 << s_59_2)
        };
        // D s_59_8: cast reint s_59_7 -> u64
        let s_59_8: u64 = (s_59_7.value() as u64);
        // C s_59_9: const #104856u : u32
        let s_59_9: u32 = 104856;
        // N s_59_10: write-reg s_59_9 <= s_59_8
        let s_59_10: () = {
            state.write_register::<u64>(s_59_9 as isize, s_59_8);
            tracer.write_register(s_59_9 as isize, s_59_8);
        };
        // N s_59_11: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var pmuevent:u16
        let s_60_0: u16 = fn_state.pmuevent;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 16u16);
        // C s_60_2: const #112u : u32
        let s_60_2: u32 = 112;
        // D s_60_3: read-reg s_60_2:u16
        let s_60_3: u16 = {
            let value = state.read_register::<u16>(s_60_2 as isize);
            tracer.read_register(s_60_2 as isize, value);
            value
        };
        // D s_60_4: cast zx s_60_3 -> bv
        let s_60_4: Bits = Bits::new(s_60_3 as u128, 16u16);
        // D s_60_5: cmp-eq s_60_1 s_60_4
        let s_60_5: bool = ((s_60_1) == (s_60_4));
        // D s_60_6: not s_60_5
        let s_60_6: bool = !s_60_5;
        // N s_60_7: branch s_60_6 b62 b61
        if s_60_6 {
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
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // S s_61_1: call Bit(s_61_0)
        let s_61_1: bool = Bit(state, tracer, s_61_0);
        // C s_61_2: const #8s : i
        let s_61_2: i128 = 8;
        // C s_61_3: const #104856u : u32
        let s_61_3: u32 = 104856;
        // D s_61_4: read-reg s_61_3:u64
        let s_61_4: u64 = {
            let value = state.read_register::<u64>(s_61_3 as isize);
            tracer.read_register(s_61_3 as isize, value);
            value
        };
        // D s_61_5: cast zx s_61_4 -> bv
        let s_61_5: Bits = Bits::new(s_61_4 as u128, 64u16);
        // C s_61_6: const #1u : u64
        let s_61_6: u64 = 1;
        // D s_61_7: bit-insert s_61_5 s_61_5 s_61_2 s_61_6
        let s_61_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_61_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_61_5.length(),
            );
            (s_61_5 & mask) | (s_61_5 << s_61_2)
        };
        // D s_61_8: cast reint s_61_7 -> u64
        let s_61_8: u64 = (s_61_7.value() as u64);
        // C s_61_9: const #104856u : u32
        let s_61_9: u32 = 104856;
        // N s_61_10: write-reg s_61_9 <= s_61_8
        let s_61_10: () = {
            state.write_register::<u64>(s_61_9 as isize, s_61_8);
            tracer.write_register(s_61_9 as isize, s_61_8);
        };
        // N s_61_11: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var pmuevent:u16
        let s_62_0: u16 = fn_state.pmuevent;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 16u16);
        // C s_62_2: const #56u : u32
        let s_62_2: u32 = 56;
        // D s_62_3: read-reg s_62_2:u16
        let s_62_3: u16 = {
            let value = state.read_register::<u16>(s_62_2 as isize);
            tracer.read_register(s_62_2 as isize, value);
            value
        };
        // D s_62_4: cast zx s_62_3 -> bv
        let s_62_4: Bits = Bits::new(s_62_3 as u128, 16u16);
        // D s_62_5: cmp-eq s_62_1 s_62_4
        let s_62_5: bool = ((s_62_1) == (s_62_4));
        // D s_62_6: not s_62_5
        let s_62_6: bool = !s_62_5;
        // N s_62_7: branch s_62_6 b64 b63
        if s_62_6 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // S s_63_1: call Bit(s_63_0)
        let s_63_1: bool = Bit(state, tracer, s_63_0);
        // C s_63_2: const #7s : i
        let s_63_2: i128 = 7;
        // C s_63_3: const #104856u : u32
        let s_63_3: u32 = 104856;
        // D s_63_4: read-reg s_63_3:u64
        let s_63_4: u64 = {
            let value = state.read_register::<u64>(s_63_3 as isize);
            tracer.read_register(s_63_3 as isize, value);
            value
        };
        // D s_63_5: cast zx s_63_4 -> bv
        let s_63_5: Bits = Bits::new(s_63_4 as u128, 64u16);
        // C s_63_6: const #1u : u64
        let s_63_6: u64 = 1;
        // D s_63_7: bit-insert s_63_5 s_63_5 s_63_2 s_63_6
        let s_63_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_63_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_63_5.length(),
            );
            (s_63_5 & mask) | (s_63_5 << s_63_2)
        };
        // D s_63_8: cast reint s_63_7 -> u64
        let s_63_8: u64 = (s_63_7.value() as u64);
        // C s_63_9: const #104856u : u32
        let s_63_9: u32 = 104856;
        // N s_63_10: write-reg s_63_9 <= s_63_8
        let s_63_10: () = {
            state.write_register::<u64>(s_63_9 as isize, s_63_8);
            tracer.write_register(s_63_9 as isize, s_63_8);
        };
        // N s_63_11: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var pmuevent:u16
        let s_64_0: u16 = fn_state.pmuevent;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 16u16);
        // C s_64_2: const #88u : u32
        let s_64_2: u32 = 88;
        // D s_64_3: read-reg s_64_2:u16
        let s_64_3: u16 = {
            let value = state.read_register::<u16>(s_64_2 as isize);
            tracer.read_register(s_64_2 as isize, value);
            value
        };
        // D s_64_4: cast zx s_64_3 -> bv
        let s_64_4: Bits = Bits::new(s_64_3 as u128, 16u16);
        // D s_64_5: cmp-eq s_64_1 s_64_4
        let s_64_5: bool = ((s_64_1) == (s_64_4));
        // D s_64_6: not s_64_5
        let s_64_6: bool = !s_64_5;
        // N s_64_7: branch s_64_6 b66 b65
        if s_64_6 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // S s_65_1: call Bit(s_65_0)
        let s_65_1: bool = Bit(state, tracer, s_65_0);
        // C s_65_2: const #7s : i
        let s_65_2: i128 = 7;
        // C s_65_3: const #104856u : u32
        let s_65_3: u32 = 104856;
        // D s_65_4: read-reg s_65_3:u64
        let s_65_4: u64 = {
            let value = state.read_register::<u64>(s_65_3 as isize);
            tracer.read_register(s_65_3 as isize, value);
            value
        };
        // D s_65_5: cast zx s_65_4 -> bv
        let s_65_5: Bits = Bits::new(s_65_4 as u128, 64u16);
        // C s_65_6: const #1u : u64
        let s_65_6: u64 = 1;
        // D s_65_7: bit-insert s_65_5 s_65_5 s_65_2 s_65_6
        let s_65_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_65_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_65_5.length(),
            );
            (s_65_5 & mask) | (s_65_5 << s_65_2)
        };
        // D s_65_8: cast reint s_65_7 -> u64
        let s_65_8: u64 = (s_65_7.value() as u64);
        // C s_65_9: const #104856u : u32
        let s_65_9: u32 = 104856;
        // N s_65_10: write-reg s_65_9 <= s_65_8
        let s_65_10: () = {
            state.write_register::<u64>(s_65_9 as isize, s_65_8);
            tracer.write_register(s_65_9 as isize, s_65_8);
        };
        // N s_65_11: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var pmuevent:u16
        let s_66_0: u16 = fn_state.pmuevent;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 16u16);
        // C s_66_2: const #128u : u32
        let s_66_2: u32 = 128;
        // D s_66_3: read-reg s_66_2:u16
        let s_66_3: u16 = {
            let value = state.read_register::<u16>(s_66_2 as isize);
            tracer.read_register(s_66_2 as isize, value);
            value
        };
        // D s_66_4: cast zx s_66_3 -> bv
        let s_66_4: Bits = Bits::new(s_66_3 as u128, 16u16);
        // D s_66_5: cmp-eq s_66_1 s_66_4
        let s_66_5: bool = ((s_66_1) == (s_66_4));
        // D s_66_6: not s_66_5
        let s_66_6: bool = !s_66_5;
        // N s_66_7: branch s_66_6 b68 b67
        if s_66_6 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // S s_67_1: call Bit(s_67_0)
        let s_67_1: bool = Bit(state, tracer, s_67_0);
        // C s_67_2: const #5s : i
        let s_67_2: i128 = 5;
        // C s_67_3: const #104856u : u32
        let s_67_3: u32 = 104856;
        // D s_67_4: read-reg s_67_3:u64
        let s_67_4: u64 = {
            let value = state.read_register::<u64>(s_67_3 as isize);
            tracer.read_register(s_67_3 as isize, value);
            value
        };
        // D s_67_5: cast zx s_67_4 -> bv
        let s_67_5: Bits = Bits::new(s_67_4 as u128, 64u16);
        // C s_67_6: const #1u : u64
        let s_67_6: u64 = 1;
        // D s_67_7: bit-insert s_67_5 s_67_5 s_67_2 s_67_6
        let s_67_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_67_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_67_5.length(),
            );
            (s_67_5 & mask) | (s_67_5 << s_67_2)
        };
        // D s_67_8: cast reint s_67_7 -> u64
        let s_67_8: u64 = (s_67_7.value() as u64);
        // C s_67_9: const #104856u : u32
        let s_67_9: u32 = 104856;
        // N s_67_10: write-reg s_67_9 <= s_67_8
        let s_67_10: () = {
            state.write_register::<u64>(s_67_9 as isize, s_67_8);
            tracer.write_register(s_67_9 as isize, s_67_8);
        };
        // N s_67_11: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var pmuevent:u16
        let s_68_0: u16 = fn_state.pmuevent;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 16u16);
        // C s_68_2: const #96u : u32
        let s_68_2: u32 = 96;
        // D s_68_3: read-reg s_68_2:u16
        let s_68_3: u16 = {
            let value = state.read_register::<u16>(s_68_2 as isize);
            tracer.read_register(s_68_2 as isize, value);
            value
        };
        // D s_68_4: cast zx s_68_3 -> bv
        let s_68_4: Bits = Bits::new(s_68_3 as u128, 16u16);
        // D s_68_5: cmp-eq s_68_1 s_68_4
        let s_68_5: bool = ((s_68_1) == (s_68_4));
        // D s_68_6: not s_68_5
        let s_68_6: bool = !s_68_5;
        // N s_68_7: branch s_68_6 b70 b69
        if s_68_6 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // S s_69_1: call Bit(s_69_0)
        let s_69_1: bool = Bit(state, tracer, s_69_0);
        // C s_69_2: const #4s : i
        let s_69_2: i128 = 4;
        // C s_69_3: const #104856u : u32
        let s_69_3: u32 = 104856;
        // D s_69_4: read-reg s_69_3:u64
        let s_69_4: u64 = {
            let value = state.read_register::<u64>(s_69_3 as isize);
            tracer.read_register(s_69_3 as isize, value);
            value
        };
        // D s_69_5: cast zx s_69_4 -> bv
        let s_69_5: Bits = Bits::new(s_69_4 as u128, 64u16);
        // C s_69_6: const #1u : u64
        let s_69_6: u64 = 1;
        // D s_69_7: bit-insert s_69_5 s_69_5 s_69_2 s_69_6
        let s_69_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_69_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_69_5.length(),
            );
            (s_69_5 & mask) | (s_69_5 << s_69_2)
        };
        // D s_69_8: cast reint s_69_7 -> u64
        let s_69_8: u64 = (s_69_7.value() as u64);
        // C s_69_9: const #104856u : u32
        let s_69_9: u32 = 104856;
        // N s_69_10: write-reg s_69_9 <= s_69_8
        let s_69_10: () = {
            state.write_register::<u64>(s_69_9 as isize, s_69_8);
            tracer.write_register(s_69_9 as isize, s_69_8);
        };
        // N s_69_11: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var pmuevent:u16
        let s_70_0: u16 = fn_state.pmuevent;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 16u16);
        // C s_70_2: const #24u : u32
        let s_70_2: u32 = 24;
        // D s_70_3: read-reg s_70_2:u16
        let s_70_3: u16 = {
            let value = state.read_register::<u16>(s_70_2 as isize);
            tracer.read_register(s_70_2 as isize, value);
            value
        };
        // D s_70_4: cast zx s_70_3 -> bv
        let s_70_4: Bits = Bits::new(s_70_3 as u128, 16u16);
        // D s_70_5: cmp-eq s_70_1 s_70_4
        let s_70_5: bool = ((s_70_1) == (s_70_4));
        // D s_70_6: not s_70_5
        let s_70_6: bool = !s_70_5;
        // N s_70_7: branch s_70_6 b74 b71
        if s_70_6 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call HaveStatisticalProfilingv1p4(s_71_0)
        let s_71_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_71_0);
        // S s_71_2: not s_71_1
        let s_71_2: bool = !s_71_1;
        // N s_71_3: branch s_71_2 b73 b72
        if s_71_2 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_72_0: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // S s_73_1: call Bit(s_73_0)
        let s_73_1: bool = Bit(state, tracer, s_73_0);
        // C s_73_2: const #3s : i
        let s_73_2: i128 = 3;
        // C s_73_3: const #104856u : u32
        let s_73_3: u32 = 104856;
        // D s_73_4: read-reg s_73_3:u64
        let s_73_4: u64 = {
            let value = state.read_register::<u64>(s_73_3 as isize);
            tracer.read_register(s_73_3 as isize, value);
            value
        };
        // D s_73_5: cast zx s_73_4 -> bv
        let s_73_5: Bits = Bits::new(s_73_4 as u128, 64u16);
        // C s_73_6: const #1u : u64
        let s_73_6: u64 = 1;
        // D s_73_7: bit-insert s_73_5 s_73_5 s_73_2 s_73_6
        let s_73_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_73_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_73_5.length(),
            );
            (s_73_5 & mask) | (s_73_5 << s_73_2)
        };
        // D s_73_8: cast reint s_73_7 -> u64
        let s_73_8: u64 = (s_73_7.value() as u64);
        // C s_73_9: const #104856u : u32
        let s_73_9: u32 = 104856;
        // N s_73_10: write-reg s_73_9 <= s_73_8
        let s_73_10: () = {
            state.write_register::<u64>(s_73_9 as isize, s_73_8);
            tracer.write_register(s_73_9 as isize, s_73_8);
        };
        // N s_73_11: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var pmuevent:u16
        let s_74_0: u16 = fn_state.pmuevent;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 16u16);
        // C s_74_2: const #136u : u32
        let s_74_2: u32 = 136;
        // D s_74_3: read-reg s_74_2:u16
        let s_74_3: u16 = {
            let value = state.read_register::<u16>(s_74_2 as isize);
            tracer.read_register(s_74_2 as isize, value);
            value
        };
        // D s_74_4: cast zx s_74_3 -> bv
        let s_74_4: Bits = Bits::new(s_74_3 as u128, 16u16);
        // D s_74_5: cmp-eq s_74_1 s_74_4
        let s_74_5: bool = ((s_74_1) == (s_74_4));
        // D s_74_6: not s_74_5
        let s_74_6: bool = !s_74_5;
        // N s_74_7: branch s_74_6 b78 b75
        if s_74_6 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call HaveStatisticalProfilingv1p4(s_75_0)
        let s_75_1: bool = HaveStatisticalProfilingv1p4(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b77 b76
        if s_75_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // S s_77_1: call Bit(s_77_0)
        let s_77_1: bool = Bit(state, tracer, s_77_0);
        // C s_77_2: const #3s : i
        let s_77_2: i128 = 3;
        // C s_77_3: const #104856u : u32
        let s_77_3: u32 = 104856;
        // D s_77_4: read-reg s_77_3:u64
        let s_77_4: u64 = {
            let value = state.read_register::<u64>(s_77_3 as isize);
            tracer.read_register(s_77_3 as isize, value);
            value
        };
        // D s_77_5: cast zx s_77_4 -> bv
        let s_77_5: Bits = Bits::new(s_77_4 as u128, 64u16);
        // C s_77_6: const #1u : u64
        let s_77_6: u64 = 1;
        // D s_77_7: bit-insert s_77_5 s_77_5 s_77_2 s_77_6
        let s_77_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_77_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_77_5.length(),
            );
            (s_77_5 & mask) | (s_77_5 << s_77_2)
        };
        // D s_77_8: cast reint s_77_7 -> u64
        let s_77_8: u64 = (s_77_7.value() as u64);
        // C s_77_9: const #104856u : u32
        let s_77_9: u32 = 104856;
        // N s_77_10: write-reg s_77_9 <= s_77_8
        let s_77_10: () = {
            state.write_register::<u64>(s_77_9 as isize, s_77_8);
            tracer.write_register(s_77_9 as isize, s_77_8);
        };
        // N s_77_11: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var pmuevent:u16
        let s_78_0: u16 = fn_state.pmuevent;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 16u16);
        // C s_78_2: const #32u : u32
        let s_78_2: u32 = 32;
        // D s_78_3: read-reg s_78_2:u16
        let s_78_3: u16 = {
            let value = state.read_register::<u16>(s_78_2 as isize);
            tracer.read_register(s_78_2 as isize, value);
            value
        };
        // D s_78_4: cast zx s_78_3 -> bv
        let s_78_4: Bits = Bits::new(s_78_3 as u128, 16u16);
        // D s_78_5: cmp-eq s_78_1 s_78_4
        let s_78_5: bool = ((s_78_1) == (s_78_4));
        // D s_78_6: not s_78_5
        let s_78_6: bool = !s_78_5;
        // N s_78_7: branch s_78_6 b80 b79
        if s_78_6 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // S s_79_1: call Bit(s_79_0)
        let s_79_1: bool = Bit(state, tracer, s_79_0);
        // C s_79_2: const #2s : i
        let s_79_2: i128 = 2;
        // C s_79_3: const #104856u : u32
        let s_79_3: u32 = 104856;
        // D s_79_4: read-reg s_79_3:u64
        let s_79_4: u64 = {
            let value = state.read_register::<u64>(s_79_3 as isize);
            tracer.read_register(s_79_3 as isize, value);
            value
        };
        // D s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 64u16);
        // C s_79_6: const #1u : u64
        let s_79_6: u64 = 1;
        // D s_79_7: bit-insert s_79_5 s_79_5 s_79_2 s_79_6
        let s_79_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_79_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_79_5.length(),
            );
            (s_79_5 & mask) | (s_79_5 << s_79_2)
        };
        // D s_79_8: cast reint s_79_7 -> u64
        let s_79_8: u64 = (s_79_7.value() as u64);
        // C s_79_9: const #104856u : u32
        let s_79_9: u32 = 104856;
        // N s_79_10: write-reg s_79_9 <= s_79_8
        let s_79_10: () = {
            state.write_register::<u64>(s_79_9 as isize, s_79_8);
            tracer.write_register(s_79_9 as isize, s_79_8);
        };
        // N s_79_11: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var pmuevent:u16
        let s_80_0: u16 = fn_state.pmuevent;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 16u16);
        // C s_80_2: const #40u : u32
        let s_80_2: u32 = 40;
        // D s_80_3: read-reg s_80_2:u16
        let s_80_3: u16 = {
            let value = state.read_register::<u16>(s_80_2 as isize);
            tracer.read_register(s_80_2 as isize, value);
            value
        };
        // D s_80_4: cast zx s_80_3 -> bv
        let s_80_4: Bits = Bits::new(s_80_3 as u128, 16u16);
        // D s_80_5: cmp-eq s_80_1 s_80_4
        let s_80_5: bool = ((s_80_1) == (s_80_4));
        // D s_80_6: not s_80_5
        let s_80_6: bool = !s_80_5;
        // N s_80_7: branch s_80_6 b82 b81
        if s_80_6 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // S s_81_1: call Bit(s_81_0)
        let s_81_1: bool = Bit(state, tracer, s_81_0);
        // C s_81_2: const #1s : i
        let s_81_2: i128 = 1;
        // C s_81_3: const #104856u : u32
        let s_81_3: u32 = 104856;
        // D s_81_4: read-reg s_81_3:u64
        let s_81_4: u64 = {
            let value = state.read_register::<u64>(s_81_3 as isize);
            tracer.read_register(s_81_3 as isize, value);
            value
        };
        // D s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 64u16);
        // C s_81_6: const #1u : u64
        let s_81_6: u64 = 1;
        // D s_81_7: bit-insert s_81_5 s_81_5 s_81_2 s_81_6
        let s_81_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_81_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_81_5.length(),
            );
            (s_81_5 & mask) | (s_81_5 << s_81_2)
        };
        // D s_81_8: cast reint s_81_7 -> u64
        let s_81_8: u64 = (s_81_7.value() as u64);
        // C s_81_9: const #104856u : u32
        let s_81_9: u32 = 104856;
        // N s_81_10: write-reg s_81_9 <= s_81_8
        let s_81_10: () = {
            state.write_register::<u64>(s_81_9 as isize, s_81_8);
            tracer.write_register(s_81_9 as isize, s_81_8);
        };
        // N s_81_11: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var pmuevent:u16
        let s_82_0: u16 = fn_state.pmuevent;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 16u16);
        // C s_82_2: const #48u : u32
        let s_82_2: u32 = 48;
        // D s_82_3: read-reg s_82_2:u16
        let s_82_3: u16 = {
            let value = state.read_register::<u16>(s_82_2 as isize);
            tracer.read_register(s_82_2 as isize, value);
            value
        };
        // D s_82_4: cast zx s_82_3 -> bv
        let s_82_4: Bits = Bits::new(s_82_3 as u128, 16u16);
        // D s_82_5: cmp-eq s_82_1 s_82_4
        let s_82_5: bool = ((s_82_1) == (s_82_4));
        // D s_82_6: not s_82_5
        let s_82_6: bool = !s_82_5;
        // N s_82_7: branch s_82_6 b84 b83
        if s_82_6 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // S s_83_1: call Bit(s_83_0)
        let s_83_1: bool = Bit(state, tracer, s_83_0);
        // C s_83_2: const #0s : i
        let s_83_2: i128 = 0;
        // C s_83_3: const #104856u : u32
        let s_83_3: u32 = 104856;
        // D s_83_4: read-reg s_83_3:u64
        let s_83_4: u64 = {
            let value = state.read_register::<u64>(s_83_3 as isize);
            tracer.read_register(s_83_3 as isize, value);
            value
        };
        // D s_83_5: cast zx s_83_4 -> bv
        let s_83_5: Bits = Bits::new(s_83_4 as u128, 64u16);
        // C s_83_6: const #1u : u64
        let s_83_6: u64 = 1;
        // D s_83_7: bit-insert s_83_5 s_83_5 s_83_2 s_83_6
        let s_83_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_83_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_83_5.length(),
            );
            (s_83_5 & mask) | (s_83_5 << s_83_2)
        };
        // D s_83_8: cast reint s_83_7 -> u64
        let s_83_8: u64 = (s_83_7.value() as u64);
        // C s_83_9: const #104856u : u32
        let s_83_9: u32 = 104856;
        // N s_83_10: write-reg s_83_9 <= s_83_8
        let s_83_10: () = {
            state.write_register::<u64>(s_83_9 as isize, s_83_8);
            tracer.write_register(s_83_9 as isize, s_83_8);
        };
        // N s_83_11: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: return
        return;
    }
}
