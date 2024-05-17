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
use u_get_MPAMIDR_EL1_Type_VPMR_MAX::*;
use mapvpmw::*;
use fmod_int::*;
use u__id::*;
use u_get_MPAMIDR_EL1_Type_PARTID_MAX::*;
use common::*;
pub fn MAP_vPARTID<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vpartid: u16,
) -> ProductType4d3ef3a5cd661176 {
    #[derive(Default)]
    struct FunctionState {
        gs_7035: bool,
        virtshadow_87: i128,
        virt: i128,
        err: bool,
        vpartid_max: i128,
        ret: u16,
        vpartid: u16,
    }
    let fn_state = FunctionState {
        vpartid,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_0_0: read-var vpartid:u16
        let s_0_0: u16 = fn_state.vpartid;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 16u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: write-var virt <= s_0_2
        fn_state.virt = s_0_2;
        // C s_0_4: const #11032u : u32
        let s_0_4: u32 = 11032;
        // D s_0_5: read-reg s_0_4:struct
        let s_0_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: call _get_MPAMIDR_EL1_Type_VPMR_MAX(s_0_5)
        let s_0_6: u8 = u_get_MPAMIDR_EL1_Type_VPMR_MAX(state, tracer, s_0_5);
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 3u16);
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (s_0_7.value() as i128);
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // C s_0_10: const #2s : i
        let s_0_10: i128 = 2;
        // D s_0_11: cast zx s_0_9 -> i
        let s_0_11: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_12: lsl s_0_11 s_0_10
        let s_0_12: i128 = s_0_11 << s_0_10;
        // C s_0_13: const #3s : i
        let s_0_13: i128 = 3;
        // D s_0_14: add s_0_12 s_0_13
        let s_0_14: i128 = (s_0_12 + s_0_13);
        // D s_0_15: write-var vpartid_max <= s_0_14
        fn_state.vpartid_max = s_0_14;
        // D s_0_16: read-var vpartid:u16
        let s_0_16: u16 = fn_state.vpartid;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 16u16);
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (s_0_17.value() as i128);
        // D s_0_19: cast reint s_0_18 -> i64
        let s_0_19: i64 = (s_0_18 as i64);
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: read-var vpartid_max:i
        let s_0_21: i128 = fn_state.vpartid_max;
        // D s_0_22: cmp-gt s_0_20 s_0_21
        let s_0_22: bool = ((s_0_20) > (s_0_21));
        // N s_0_23: branch s_0_22 b14 b1
        if s_0_22 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_2_0: read-var virt:i
        let s_2_0: i128 = fn_state.virt;
        // D s_2_1: write-var virtshadow#87 <= s_2_0
        fn_state.virtshadow_87 = s_2_0;
        // D s_2_2: read-var virtshadow#87:i
        let s_2_2: i128 = fn_state.virtshadow_87;
        // D s_2_3: call __id(s_2_2)
        let s_2_3: i128 = u__id(state, tracer, s_2_2);
        // C s_2_4: const #0s : i
        let s_2_4: i128 = 0;
        // D s_2_5: cmp-le s_2_4 s_2_3
        let s_2_5: bool = ((s_2_4) <= (s_2_3));
        // N s_2_6: branch s_2_5 b13 b3
        if s_2_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#7035 <= s_3_0
        fn_state.gs_7035 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_4_0: read-var gs#7035:u8
        let s_4_0: bool = fn_state.gs_7035;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #17312u : u32
        let s_4_2: u32 = 17312;
        // D s_4_3: read-reg s_4_2:u64
        let s_4_3: u64 = {
            let value = state.read_register::<u64>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 64u16);
        // D s_4_5: read-var virtshadow#87:i
        let s_4_5: i128 = fn_state.virtshadow_87;
        // C s_4_6: const #1u : u64
        let s_4_6: u64 = 1;
        // D s_4_7: bit-extract s_4_4 s_4_5 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_4) >> (s_4_5)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: bool = ((s_4_7.value()) != 0);
        // C s_4_9: const #0s : i
        let s_4_9: i128 = 0;
        // C s_4_10: const #0u : u64
        let s_4_10: u64 = 0;
        // D s_4_11: cast zx s_4_8 -> u64
        let s_4_11: u64 = (s_4_8 as u64);
        // C s_4_12: const #1u : u64
        let s_4_12: u64 = 1;
        // D s_4_13: and s_4_11 s_4_12
        let s_4_13: u64 = ((s_4_11) & (s_4_12));
        // D s_4_14: cmp-eq s_4_13 s_4_12
        let s_4_14: bool = ((s_4_13) == (s_4_12));
        // D s_4_15: lsl s_4_11 s_4_9
        let s_4_15: u64 = s_4_11 << s_4_9;
        // D s_4_16: or s_4_10 s_4_15
        let s_4_16: u64 = ((s_4_10) | (s_4_15));
        // D s_4_17: cmpl s_4_15
        let s_4_17: u64 = !s_4_15;
        // D s_4_18: and s_4_10 s_4_17
        let s_4_18: u64 = ((s_4_10) & (s_4_17));
        // D s_4_19: select s_4_14 s_4_16 s_4_18
        let s_4_19: u64 = if s_4_14 { s_4_16 } else { s_4_18 };
        // D s_4_20: cast trunc s_4_19 -> u8
        let s_4_20: bool = ((s_4_19) != 0);
        // D s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 1u16);
        // C s_4_22: const #1u : u8
        let s_4_22: bool = true;
        // C s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 1u16);
        // D s_4_24: cmp-eq s_4_21 s_4_23
        let s_4_24: bool = ((s_4_21) == (s_4_23));
        // N s_4_25: branch s_4_24 b12 b5
        if s_4_24 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // C s_5_0: const #17312u : u32
        let s_5_0: u32 = 17312;
        // D s_5_1: read-reg s_5_0:u64
        let s_5_1: u64 = {
            let value = state.read_register::<u64>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 64u16);
        // C s_5_4: const #1u : u64
        let s_5_4: u64 = 1;
        // D s_5_5: bit-extract s_5_3 s_5_2 s_5_4
        let s_5_5: Bits = (Bits::new(
            ((s_5_3) >> (s_5_2)).value(),
            u16::try_from(s_5_4).unwrap(),
        ));
        // D s_5_6: cast reint s_5_5 -> u8
        let s_5_6: bool = ((s_5_5.value()) != 0);
        // C s_5_7: const #0s : i
        let s_5_7: i128 = 0;
        // C s_5_8: const #0u : u64
        let s_5_8: u64 = 0;
        // D s_5_9: cast zx s_5_6 -> u64
        let s_5_9: u64 = (s_5_6 as u64);
        // C s_5_10: const #1u : u64
        let s_5_10: u64 = 1;
        // D s_5_11: and s_5_9 s_5_10
        let s_5_11: u64 = ((s_5_9) & (s_5_10));
        // D s_5_12: cmp-eq s_5_11 s_5_10
        let s_5_12: bool = ((s_5_11) == (s_5_10));
        // D s_5_13: lsl s_5_9 s_5_7
        let s_5_13: u64 = s_5_9 << s_5_7;
        // D s_5_14: or s_5_8 s_5_13
        let s_5_14: u64 = ((s_5_8) | (s_5_13));
        // D s_5_15: cmpl s_5_13
        let s_5_15: u64 = !s_5_13;
        // D s_5_16: and s_5_8 s_5_15
        let s_5_16: u64 = ((s_5_8) & (s_5_15));
        // D s_5_17: select s_5_12 s_5_14 s_5_16
        let s_5_17: u64 = if s_5_12 { s_5_14 } else { s_5_16 };
        // D s_5_18: cast trunc s_5_17 -> u8
        let s_5_18: bool = ((s_5_17) != 0);
        // D s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 1u16);
        // C s_5_20: const #1u : u8
        let s_5_20: bool = true;
        // C s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 1u16);
        // D s_5_22: cmp-eq s_5_19 s_5_21
        let s_5_22: bool = ((s_5_19) == (s_5_21));
        // N s_5_23: branch s_5_22 b11 b6
        if s_5_22 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // C s_6_0: const #768u : u32
        let s_6_0: u32 = 768;
        // D s_6_1: read-reg s_6_0:u16
        let s_6_1: u16 = {
            let value = state.read_register::<u16>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: write-var ret <= s_6_1
        fn_state.ret = s_6_1;
        // C s_6_3: const #1u : u8
        let s_6_3: bool = true;
        // D s_6_4: write-var err <= s_6_3
        fn_state.err = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // C s_7_0: const #11032u : u32
        let s_7_0: u32 = 11032;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_MPAMIDR_EL1_Type_PARTID_MAX(s_7_1)
        let s_7_2: u16 = u_get_MPAMIDR_EL1_Type_PARTID_MAX(state, tracer, s_7_1);
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 16u16);
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (s_7_3.value() as i128);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var ret:u16
        let s_7_6: u16 = fn_state.ret;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 16u16);
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (s_7_7.value() as i128);
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: cast zx s_7_5 -> i
        let s_7_11: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_12: cmp-gt s_7_10 s_7_11
        let s_7_12: bool = ((s_7_10) > (s_7_11));
        // N s_7_13: branch s_7_12 b10 b8
        if s_7_12 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_9_0: read-var err:u8
        let s_9_0: bool = fn_state.err;
        // D s_9_1: read-var ret:u16
        let s_9_1: u16 = fn_state.ret;
        // D s_9_2: create-product struct = ["s_9_1", "s_9_0"]
        let s_9_2: ProductType4d3ef3a5cd661176 = ProductType4d3ef3a5cd661176 {
            _0: s_9_1,
            _1: s_9_0,
        };
        // N s_9_3: return s_9_2
        return s_9_2;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // C s_10_0: const #768u : u32
        let s_10_0: u32 = 768;
        // D s_10_1: read-reg s_10_0:u16
        let s_10_1: u16 = {
            let value = state.read_register::<u16>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: write-var ret <= s_10_1
        fn_state.ret = s_10_1;
        // C s_10_3: const #1u : u8
        let s_10_3: bool = true;
        // D s_10_4: write-var err <= s_10_3
        fn_state.err = s_10_3;
        // N s_10_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // C s_11_0: const #100224u : u32
        let s_11_0: u32 = 100224;
        // D s_11_1: read-reg s_11_0:u64
        let s_11_1: u64 = {
            let value = state.read_register::<u64>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // C s_11_2: const #0s : i
        let s_11_2: i128 = 0;
        // C s_11_3: const #16s : i
        let s_11_3: i128 = 16;
        // D s_11_4: cast zx s_11_1 -> bv
        let s_11_4: Bits = Bits::new(s_11_1 as u128, 64u16);
        // D s_11_5: bit-extract s_11_4 s_11_2 s_11_3
        let s_11_5: Bits = (Bits::new(
            ((s_11_4) >> (s_11_2)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_6: cast reint s_11_5 -> u16
        let s_11_6: u16 = (s_11_5.value() as u16);
        // D s_11_7: write-var ret <= s_11_6
        fn_state.ret = s_11_6;
        // C s_11_8: const #0u : u8
        let s_11_8: bool = false;
        // D s_11_9: write-var err <= s_11_8
        fn_state.err = s_11_8;
        // N s_11_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_12_0: read-var virtshadow#87:i
        let s_12_0: i128 = fn_state.virtshadow_87;
        // D s_12_1: call mapvpmw(s_12_0)
        let s_12_1: u16 = mapvpmw(state, tracer, s_12_0);
        // D s_12_2: write-var ret <= s_12_1
        fn_state.ret = s_12_1;
        // C s_12_3: const #0u : u8
        let s_12_3: bool = false;
        // D s_12_4: write-var err <= s_12_3
        fn_state.err = s_12_3;
        // N s_12_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_13_0: read-var virtshadow#87:i
        let s_13_0: i128 = fn_state.virtshadow_87;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // C s_13_2: const #64s : i
        let s_13_2: i128 = 64;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // D s_13_4: write-var gs#7035 <= s_13_3
        fn_state.gs_7035 = s_13_3;
        // N s_13_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // C s_14_0: const #1s : i
        let s_14_0: i128 = 1;
        // D s_14_1: read-var vpartid_max:i
        let s_14_1: i128 = fn_state.vpartid_max;
        // D s_14_2: add s_14_1 s_14_0
        let s_14_2: i128 = (s_14_1 + s_14_0);
        // D s_14_3: read-var virt:i
        let s_14_3: i128 = fn_state.virt;
        // D s_14_4: call fmod_int(s_14_3, s_14_2)
        let s_14_4: i128 = fmod_int(state, tracer, s_14_3, s_14_2);
        // D s_14_5: write-var virt <= s_14_4
        fn_state.virt = s_14_4;
        // N s_14_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
