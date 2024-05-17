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
use UsingAArch32::*;
use u__id::*;
use HavePMUv3::*;
use PMEVTYPER_read::*;
use u_get_PMEVTYPER_Type_evtCount::*;
use u_get_PMEVTYPER_EL0_Type_evtCount::*;
use common::*;
pub fn PMUEvent__2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pmuevent: u16,
    increment_name: i128,
    idx: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_2724: bool,
        gs_2719: bool,
        gs_2715: bool,
        pmuevent: u16,
        increment_name: i128,
        idx: i128,
    }
    let fn_state = FunctionState {
        pmuevent,
        increment_name,
        idx,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HavePMUv3(s_0_0)
        let s_0_1: bool = HavePMUv3(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b17 b1
        if s_0_2 {
            return block_17(state, tracer, fn_state);
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
        // S s_1_1: call UsingAArch32(s_1_0)
        let s_1_1: bool = UsingAArch32(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b11 b2
        if s_1_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var idx:i
        let s_2_0: i128 = fn_state.idx;
        // D s_2_1: call __id(s_2_0)
        let s_2_1: i128 = u__id(state, tracer, s_2_0);
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: cmp-le s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) <= (s_2_1));
        // N s_2_4: branch s_2_3 b10 b3
        if s_2_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#2715 <= s_3_0
        fn_state.gs_2715 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#2715:u8
        let s_4_0: bool = fn_state.gs_2715;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #11208u : u32
        let s_4_2: u32 = 11208;
        // D s_4_3: read-reg s_4_2:[struct; 32]
        let s_4_3: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 32usize]>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: read-var idx:i
        let s_4_4: i128 = fn_state.idx;
        // D s_4_5: read-element s_4_3[s_4_4]
        let s_4_5: ProductType5c790c8ef59cc8b2 = s_4_3[(s_4_4) as usize];
        // D s_4_6: call _get_PMEVTYPER_EL0_Type_evtCount(s_4_5)
        let s_4_6: u16 = u_get_PMEVTYPER_EL0_Type_evtCount(state, tracer, s_4_5);
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 16u16);
        // D s_4_8: read-var pmuevent:u16
        let s_4_8: u16 = fn_state.pmuevent;
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 16u16);
        // D s_4_10: cmp-eq s_4_7 s_4_9
        let s_4_10: bool = ((s_4_7) == (s_4_9));
        // N s_4_11: branch s_4_10 b6 b5
        if s_4_10 {
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
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var idx:i
        let s_6_0: i128 = fn_state.idx;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: const #0s : i
        let s_6_3: i128 = 0;
        // D s_6_4: cast zx s_6_2 -> i
        let s_6_4: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_5: cmp-le s_6_3 s_6_4
        let s_6_5: bool = ((s_6_3) <= (s_6_4));
        // N s_6_6: branch s_6_5 b9 b7
        if s_6_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#2719 <= s_7_0
        fn_state.gs_2719 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#2719:u8
        let s_8_0: bool = fn_state.gs_2719;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // C s_8_2: const #89632u : u32
        let s_8_2: u32 = 89632;
        // D s_8_3: read-reg s_8_2:[i; 31]
        let s_8_3: [i128; 31usize] = {
            let value = state.read_register::<[i128; 31usize]>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: read-var idx:i
        let s_8_4: i128 = fn_state.idx;
        // D s_8_5: read-element s_8_3[s_8_4]
        let s_8_5: i128 = s_8_3[(s_8_4) as usize];
        // D s_8_6: read-var increment_name:i
        let s_8_6: i128 = fn_state.increment_name;
        // D s_8_7: add s_8_5 s_8_6
        let s_8_7: i128 = (s_8_5 + s_8_6);
        // C s_8_8: const #89632u : u32
        let s_8_8: u32 = 89632;
        // D s_8_9: read-reg s_8_8:[i; 31]
        let s_8_9: [i128; 31usize] = {
            let value = state.read_register::<[i128; 31usize]>(s_8_8 as isize);
            tracer.read_register(s_8_8 as isize, value);
            value
        };
        // D s_8_10: read-var idx:i
        let s_8_10: i128 = fn_state.idx;
        // D s_8_11: mutate-element s_8_9[s_8_10] <= s_8_7
        let s_8_11: [i128; 31usize] = {
            let mut local = s_8_9.clone();
            local[(s_8_10) as usize] = s_8_7;
            local
        };
        // D s_8_12: cast cvt s_8_11 -> [i; 0]
        let s_8_12: alloc::vec::Vec<i128> = alloc::vec::Vec::from(s_8_11);
        // D s_8_13: cast cvt s_8_12 -> [i; 31]
        let s_8_13: [i128; 31usize] = {
            let mut buf = [Default::default(); 31usize];
            buf.copy_from_slice(&s_8_12);
            buf
        };
        // C s_8_14: const #89632u : u32
        let s_8_14: u32 = 89632;
        // N s_8_15: write-reg s_8_14 <= s_8_13
        let s_8_15: () = {
            state.write_register::<[i128; 31usize]>(s_8_14 as isize, s_8_13);
            tracer.write_register(s_8_14 as isize, s_8_13);
        };
        // N s_8_16: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var idx:i
        let s_9_0: i128 = fn_state.idx;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // C s_9_3: const #31s : i
        let s_9_3: i128 = 31;
        // D s_9_4: cast zx s_9_2 -> i
        let s_9_4: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_5: cmp-lt s_9_4 s_9_3
        let s_9_5: bool = ((s_9_4) < (s_9_3));
        // D s_9_6: write-var gs#2719 <= s_9_5
        fn_state.gs_2719 = s_9_5;
        // N s_9_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var idx:i
        let s_10_0: i128 = fn_state.idx;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #32s : i
        let s_10_2: i128 = 32;
        // D s_10_3: cmp-lt s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) < (s_10_2));
        // D s_10_4: write-var gs#2715 <= s_10_3
        fn_state.gs_2715 = s_10_3;
        // N s_10_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var idx:i
        let s_11_0: i128 = fn_state.idx;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // C s_11_2: const #0s : i
        let s_11_2: i128 = 0;
        // D s_11_3: cmp-le s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) <= (s_11_1));
        // N s_11_4: branch s_11_3 b16 b12
        if s_11_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#2724 <= s_12_0
        fn_state.gs_2724 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#2724:u8
        let s_13_0: bool = fn_state.gs_2724;
        // N s_13_1: assert s_13_0
        let s_13_1: () = assert!(s_13_0);
        // D s_13_2: read-var idx:i
        let s_13_2: i128 = fn_state.idx;
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // D s_13_4: call PMEVTYPER_read(s_13_3)
        let s_13_4: ProductType700c18a878c5601b = PMEVTYPER_read(state, tracer, s_13_3);
        // D s_13_5: call _get_PMEVTYPER_Type_evtCount(s_13_4)
        let s_13_5: u16 = u_get_PMEVTYPER_Type_evtCount(state, tracer, s_13_4);
        // D s_13_6: cast zx s_13_5 -> bv
        let s_13_6: Bits = Bits::new(s_13_5 as u128, 16u16);
        // D s_13_7: read-var pmuevent:u16
        let s_13_7: u16 = fn_state.pmuevent;
        // D s_13_8: cast zx s_13_7 -> bv
        let s_13_8: Bits = Bits::new(s_13_7 as u128, 16u16);
        // D s_13_9: cmp-eq s_13_6 s_13_8
        let s_13_9: bool = ((s_13_6) == (s_13_8));
        // N s_13_10: branch s_13_9 b15 b14
        if s_13_9 {
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
        // C s_15_0: const #89632u : u32
        let s_15_0: u32 = 89632;
        // D s_15_1: read-reg s_15_0:[i; 31]
        let s_15_1: [i128; 31usize] = {
            let value = state.read_register::<[i128; 31usize]>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: read-var idx:i
        let s_15_2: i128 = fn_state.idx;
        // D s_15_3: read-element s_15_1[s_15_2]
        let s_15_3: i128 = s_15_1[(s_15_2) as usize];
        // D s_15_4: read-var increment_name:i
        let s_15_4: i128 = fn_state.increment_name;
        // D s_15_5: add s_15_3 s_15_4
        let s_15_5: i128 = (s_15_3 + s_15_4);
        // C s_15_6: const #89632u : u32
        let s_15_6: u32 = 89632;
        // D s_15_7: read-reg s_15_6:[i; 31]
        let s_15_7: [i128; 31usize] = {
            let value = state.read_register::<[i128; 31usize]>(s_15_6 as isize);
            tracer.read_register(s_15_6 as isize, value);
            value
        };
        // D s_15_8: read-var idx:i
        let s_15_8: i128 = fn_state.idx;
        // D s_15_9: mutate-element s_15_7[s_15_8] <= s_15_5
        let s_15_9: [i128; 31usize] = {
            let mut local = s_15_7.clone();
            local[(s_15_8) as usize] = s_15_5;
            local
        };
        // D s_15_10: cast cvt s_15_9 -> [i; 0]
        let s_15_10: alloc::vec::Vec<i128> = alloc::vec::Vec::from(s_15_9);
        // D s_15_11: cast cvt s_15_10 -> [i; 31]
        let s_15_11: [i128; 31usize] = {
            let mut buf = [Default::default(); 31usize];
            buf.copy_from_slice(&s_15_10);
            buf
        };
        // C s_15_12: const #89632u : u32
        let s_15_12: u32 = 89632;
        // N s_15_13: write-reg s_15_12 <= s_15_11
        let s_15_13: () = {
            state.write_register::<[i128; 31usize]>(s_15_12 as isize, s_15_11);
            tracer.write_register(s_15_12 as isize, s_15_11);
        };
        // N s_15_14: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var idx:i
        let s_16_0: i128 = fn_state.idx;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // C s_16_2: const #31s : i
        let s_16_2: i128 = 31;
        // D s_16_3: cmp-lt s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) < (s_16_2));
        // D s_16_4: write-var gs#2724 <= s_16_3
        fn_state.gs_2724 = s_16_3;
        // N s_16_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: return
        return;
    }
}
